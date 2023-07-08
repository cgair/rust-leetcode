// 问题描述:
// 海量数据中找出前 k 大数 (top k)
// 
// 问1: 有 10000000 (千万)个记录, 这些查询串的重复度比较高, 如果除去重复后, 不超过 3000000 个.
//      一个查询串的重复度越高, 说明查询它的用户越多 (越热门). 
//      请统计最热门的10个查询串, 要求使用的内存不能超过 1GB.
// 
// 问2: 10 亿个整数找出重复次数最多的 100 个整数.
// 问3: 有 1000 万个身份证号以及他们对应的数据, z身份证号可能重复, 找出出现次数最多的身份证号.
// 问4: 有 10 个文件, 每个文件 1 GB, 每个文件的每一行存放的都是用户的 query, 每个文件的 query 都可能重复.
//     按照 query 的频度排序.
// 
// 针对 top k 类问题, 
// 
// 最容易想到的方法是将数据全部排序 (快排 - O(nlongn)), 但是很显然是不能一次将全部数据读入内存进行排序的.
// 其实即使内存能够满足要求, 将所有的元素都排序做了很多的无用功 (因为只求 top k)
// 
// 通常比较好的方案是 分治 + Trie树/hash + 小顶堆 (最小堆).
// 即:
// 1. 分割数据集: 将海量数据集划分为若干个较小的子集. 每个子集的大小要足够小 (可完全加载到内存中处理)
// 2. 构建 Trie 树/哈希表: 对每个子集进行处理. 如果数据集的元素是字符串类型, 可使用Trie树来进行处理; 
//                       如果元素是数值型或其他类型, 可使用哈希表.
//                       对于每个子集, 遍历其中的元素, 并将元素插入到 Trie 树或哈希表中.
// 3. 维护小顶堆: 创建一个大小为 k 的小顶堆. 初始时, 堆中没有元素.
// 4. 遍历子集: 遍历每个子集. 对于每个元素, 执行:
//    1) 如果堆中的元素数量小于 k, 将当前元素插入堆中.
//    2) 如果堆已经有 k 个元素且当前元素大于堆顶元素, 则将堆顶元素替换为当前元素, 并进行堆调整保持小顶堆的性质
//    3) 如果当前元素小于等于堆顶元素, 则继续遍历下一个元素.
// 5. 合并堆: 在处理完所有子集后, 堆中的元素即为整个数据集的 Top K 元素. 
#include <cstdlib>
#include <iostream>

#define N 1000;

class MinHeap {
    int* data;
    int capacity;
    int pos;
public:
    MinHeap() {
        capacity = N;
        pos = 0;
        data = (int*)malloc(sizeof(int) * capacity);
        data[0] = 0;
    }
    ~MinHeap() { delete data; }

    void push(int val);
    void pop();

    int top();

    bool is_empty() {
        return pos == 0;
    }

    bool is_full() {
        return pos == capacity;
    }

    void print() {
        for (int i = 0; i < pos; ++i) {
            std::cout << data[i] << " ";
        }
        std::cout << std::endl;
    }
};

void MinHeap::push(int val) {
    if (!is_full()) {
        data[++pos] = val;
        int p = pos;
        while (p > 1 && data[p] < data[p / 2]) {
            std::swap(data[p], data[p / 2]);
            p = p / 2;
        }
    } else {
        std::cerr << "Heap is full.\n";
    }
}

int MinHeap::top() {
    if (!is_empty()) return data[1];
    else std::cerr << "Heap is empty.\n";
    return -1;
}

void MinHeap::pop() {
    if (!is_empty()) {
        data[1] = data[pos--];
        int i = 1;
        while (2 * i <= pos) {
            int son = 2 * i;
            if (son < pos && data[son] > data[son + 1]) son ++;
            if (data[i] > data[son]) {
                std::swap(data[i], data[son]);
                i = son;
            } else break;
        }
    }
}

int main()
{
    MinHeap min_heap;
    for (int i = 10; i >= 1; i--) min_heap.push(i);
    min_heap.print();
    std::cout << min_heap.top() << std::endl;
    min_heap.pop();
    std::cout << min_heap.top() << std::endl;
    min_heap.pop();
    std::cout << min_heap.top() << std::endl;
    min_heap.pop();
    std::cout << min_heap.top() << std::endl;
    min_heap.pop();

    return 0;
}