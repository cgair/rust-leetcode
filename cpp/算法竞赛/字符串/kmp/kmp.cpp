// [KMP 算法](https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm)
// 一种字符串匹配算法
// 可在 O(n+m) 时间复杂度内实现两个字符串的匹配.
// 
// 字符串匹配问题: "字符串 P 是否为字符串 S 的子串? 如果是, 它出现在 S 的哪些位置?"
// 应用太常见了: 比如有一份名单, 你急切地想知道自己在不在名单上;
// 又如一份文献, 你希望快速地找到某个关键字 (keyword）所在的章节.
#include <iostream>
#include <string>
#include <vector>

// 1. Brute-Force
int bruteForceSearch(std::string p, std::string s) {
    int m = p.length(), n = s.length();
    for (int i = 0; i < n - m; ++i) {
        int j;
        for (j = 0; i < m; ++j) {
            if (p[j] != s[i + j]) break;
        }
        // p 全都匹配了
        if (j == m) { return i; }
    }

    // s 中不存在 p 子串
    return -1;
}

// Brute-Force 改进思路:
// 尽可能利用残余的信息, 是KMP算法的思想所在.
// 如果把模式串 (P) 视为一把标尺, 在主串 (S) 上移动, 那么 
// Brute-Force 就是每次失败之后只右移一位; 
// 实际上每次失败之后, 移很多位, 跳过那些不可能匹配成功的位置.
// 如何确定要移多少位呢? (如何 "跳过不可能成功的尝试"?)
// 
// 1. 针对模式串的 next[] 数组
// next[i]: 表示模式串前 i 个字符组成的子串 (P[0] ~ P[i - 1])
//          其最长的相同 真前缀 和相同 真后缀 的长度. 
//          真前缀/后缀:除去整个字符串本身的前缀和后缀.
// EXAMLE: P = "ababaca", length = 7. next[0] = -1, next[1] = 0, next[2] = 0, next[3] = 1.
// <https://picx.zhimg.com/80/v2-6ddb50d021e9fa660b5add8ea225383a_1440w.webp?source=1940ef5c>
// 
// 1.1 如何快速构建 next[] 数组?
std::vector<int> genNext(std::string& p) {
    std::vector<int> next(p.length() + 1, 0);
    next[0] = -1;
    next[1] = 0;
    int j = 0;
    for (int i = 1; i < p.length(); ++i) {
        while (j > 0 && p[i] != p[j]) {
            j = next[j];
        }
        if (p[i] == p[j]) j++;
        next[i + 1] = j;
    }

    return next;
}

// 1.2
int search(std::string& p, std::string& s) {
    int ppos = 0, spos = 0;
    
    while (spos < s.length()) {
        if (s[spos] == p[ppos]) {
            spos ++;
            ppos ++;
        } else if (ppos > 0) {

        } else {
            spos ++;
        }
    }
}


void printVec(std::vector<int>& vec) {
    std::cout << "[";
    for (auto v : vec) {
        std::cout << v << ((v != vec.back()) ? ", " : "");
    }
    std::cout << "]\n";
}

int main() 
{   
    std::string s1 = "tobeornottobe", p1 = "ob";
    std::cout << bruteForceSearch(p1, s1) << std::endl;

    // 不难想到 Brute-Force 算法所面对的最坏情况
    std::string worst_s = "AAAAAAAAAAAAAAAAAAAAAAB", worst_p = "AAAAAAAAAAAAAAB";
    std::cout << bruteForceSearch(worst_p, worst_s) << std::endl;
    // 每次字符串比较都需要 |P| 次字符比较, 共需要比较 |S| - |P| + 1次
    // 时间复杂度是 O((|S| - |P| + 1) * |P|)
    // i.e., O(nm)
    // 说白了就是字符串中重复的字符比较多, Brute-Force就显得很蠢.

    // test genNext()
    std::vector<int> ret1 = genNext(p1);
    std::vector<int> ret2 = genNext(worst_p);
    printVec(ret1);
    printVec(ret2);


    return 0;
}