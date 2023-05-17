/**
 * Hash Table
 * 
 * 1. 设计出好的散列函数
 * 1) 一致性散列的假设 (一个好的散列函数应满足):
 *    每个关键字都等可能的散列到 m 个槽位的任何一个之中, 
 *    并与其他的关键字已被散列到哪一个槽位无关.
 * 2) 方法
 *    直接寻址法: 键的全集比较小, 直接使用键作为数组的下标.
 *    除法散列法: h(k) = k mod m, m 不应是 2 的幂 (2^p), 否则 h(k) 就是 k 的 p 个最低位数字 
 *              (Recall: 如果 n 是 2 的幂次方, 则可以使用位运算 'a & (n - 1)' 来替代取模运算 'a % n')
 *    乘法散列法:
 *    平方取中法: 将键 k 平方, 然后取结果的中间几位作为哈希值 (适用于 k 的分布不均匀)
 *    折叠法: 将输入划分为相等长度的部分 (最后一部分可以不等长), 然后将这些部分相加, 以得到哈希值.
 * 
 * 
 * 2. 解决碰撞 (collision)
 * 1) 链接法 (chaining)
 * 2) 开放寻址法 (open addressing)
 *    线性探查 (linear probing): 散列函数包含探查号 --> h(k, i) = (h'(k) + i) mod m, i = 0, 1, 2, ..., m-1 --> 一次群集
 *    二次探查 (quadratic probing): 偏移量以二次方式依赖于探查号 --> h(k, i) = (h'(k) + c1 * i + c2 * i^2) mod m, i = 0, 1, 2, ..., m-1 --> 二次群集
 *    双重散列: h(k, i) = (h1(k) + i * h2(k)) mod m
 * 
 * 
*/
#include <iostream>

#define SUCCESS 0
#define FAIL 1
#define OVERFLOW -1
#define MAXNUM 9999		// 用于初始化哈希表的记录 key

typedef int Status;
typedef int Key;
typedef int Value;

typedef struct {
    Key key;
    Value value;
}RcdType;

typedef struct
{
    RcdType* rcd;
    int size;
    int count;
    int* tag;
}HashTable;

// 哈希表每次重建增长后的大小
int hashsize[] = { 11, 31, 61, 127, 251, 503 };
int idx = 0;

// 除法散列法
int hash(Key key, int m) {
    return (3 * key) % m;   // an attempt to "spread out" the keys over the hash table
}

// 处理哈希冲突: 线性探测
void handleCollision(int &p, int m) {
	p = (p + 1) % m;
}

Status init(HashTable& HT, int size) {
    HT.rcd = (RcdType*)malloc(sizeof(RcdType) * size);
    HT.tag = (int*)malloc(sizeof(int) * size);
    if (HT.rcd == nullptr || HT.tag == nullptr) return OVERFLOW;
    Key maxnum = MAXNUM;
    for (int i = 0; i < size; ++i) {
        HT.tag[i] = 0;
        HT.rcd[i].key = maxnum;
        HT.rcd[i].value = 0;
    }
    HT.size = size;
    HT.count = 0;

    return SUCCESS;
}

Status search(HashTable& HT, Key key, int& h, int& c) {
    h = hash(key, HT.size);
    c = 0;
    if (HT.tag[h] == 1 && HT.rcd[h].key == key) return SUCCESS;
    while ((HT.tag[h] == 1 && HT.rcd[h].key != key) || HT.tag[h] == -1) {
        handleCollision(h, HT.size);
        c++;
    }
    
    return FAIL;
}

// 函数声明: 插入哈希表
Status insert(HashTable& HT, Key key, Value val);

Status recreate(HashTable& HT) {
    RcdType* old_rcd;
    int* old_tag;
    int old_size;
    old_rcd = HT.rcd;
    old_tag = HT.tag;
    old_size = HT.size;

    init(HT, hashsize[idx++]);

    for (int i = 0; i < old_size; ++i) {
        if (old_tag[i] == 1) {
            insert(HT, old_rcd[i].key, old_rcd[i].value);
        }
    }

    return SUCCESS;
}

Status insert(HashTable& HT, Key key, Value val) {
    int h, c;

    if (search(HT, key, h, c) == FAIL) {
        if (c*1.0 / HT.size < 0.5) {
            HT.rcd[h].key = key;
            HT.rcd[h].value = val;
            HT.tag[h] = 1;
            HT.count ++;
            return SUCCESS;
        }
    } else {
        // 重构哈希表
        recreate(HT);
    }
    return FAIL;
}

Status remove(HashTable& HT, Key key) {
    int p, c;
    if (search(HT, key, p, c) == SUCCESS) {
        HT.tag[p] = -1;
        HT.count --;
        return SUCCESS;
    }
    else return FAIL;
}

void print(HashTable& HT) {
    for (int i = 0; i < HT.size; ++i) {
        std::cout << "TAG: " << HT.tag[i] << "; "
                  << "KEY: " << HT.rcd[i].key << ", "
                  << "VALUE: " << HT.rcd[i].value << std::endl;
    }
}

int main()
{   
    std::cout << "====================\n";
    std::cout << "     hash table     \n";
    std::cout << "====================\n";

    HashTable HT;

    if (init(HT, hashsize[idx++]) == SUCCESS) std::cout << "Successful initialization" << std::endl;

    std::cout << "\nInsertion\n";
    // In C++11, you should use std::make_pair to initialize std::pair objects
    // std::pair<Key, Value> nums[8] = {
    //     {22, 1},
    //     {41, 2},
    //     {53, 3},
    //     {46, 4},
    //     {30, 5},
    //     {13, 6},
    //     {12, 7},
    //     {67, 8},
    // };
    std::pair<Key, Value> nums[8] = {
    std::make_pair(22, 1),
    std::make_pair(41, 2),
    std::make_pair(53, 3),
    std::make_pair(46, 4),
    std::make_pair(30, 5),
    std::make_pair(13, 6),
    std::make_pair(12, 7),
    std::make_pair(67, 8),
    };

    for (int i = 0; i < 8; ++i) {
        insert(HT, nums[i].first, nums[i].second);
        print(HT);
    }

    std::cout << "\nDeletion\n";
    if (remove(HT, 12) == SUCCESS) std::cout << "delete 12\n";
    print(HT);

	//再次插入, 测试哈希表的重建
    std::pair<Key, Value> substitute[8] = {
    std::make_pair(27, 11),
    std::make_pair(47, 12),
    std::make_pair(57, 13),
    std::make_pair(47, 14),
    std::make_pair(37, 15),
    std::make_pair(17, 16),
    std::make_pair(93, 17),
    std::make_pair(67, 18),
    };

	for (int i = 0; i <= 7; i++) {
        insert(HT, substitute[i].first, substitute[i].second);
        print(HT);
	}

    getchar();

    return 0;
}