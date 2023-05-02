// problem: https://leetcode.cn/problems/find-all-anagrams-in-a-string/
// 示例 1:
// 输入: s = "cbaebabacd", p = "abc"
// 输出: [0,6]
// 解释:
// 起始索引等于 0 的子串是 "cba", 它是 "abc" 的异位词.
// 起始索引等于 6 的子串是 "bac", 它是 "abc" 的异位词.
// 
// 
// 示例 2:
// 输入: s = "abab", p = "ab"
// 输出: [0,1,2]
// 解释:
// 起始索引等于 0 的子串是 "ab", 它是 "ab" 的异位词.
// 起始索引等于 1 的子串是 "ba", 它是 "ab" 的异位词.
// 起始索引等于 2 的子串是 "ab", 它是 "ab" 的异位词.
// 
#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>
using namespace std;


class Solution {
public:
    vector<int> findAnagrams(string s, string t) {
        vector<int> ret;
        unordered_map<char, int> needs, umap;
        for (auto ch : t) { needs[ch] ++; }
        int sz = s.size();
        int left = 0, right = 0;
        int valid = 0;
        while (right < sz) {
            char rc = s[right];
            right ++;
            if (needs.count(rc)) {
                umap[rc] ++;
                if (umap[rc] == needs[rc]) valid ++;
            }

            while (right - left >= t.size()) {
                if (valid == needs.size()) ret.push_back(left);     // ❗️ 注意
                char lc = s[left];
                left ++;
                if (needs.count(lc)) {
                    if (needs[lc] == umap[lc]) valid --;
                    umap[lc] --;
                }
            }
        }


        return ret;
    }
};


int main() 
{
    Solution s;
    vector<int> ret = s.findAnagrams("baa", "aa");
    for (auto &v : ret) {
        cout << v << " ";
    }
    cout << endl;
    return 0;
}