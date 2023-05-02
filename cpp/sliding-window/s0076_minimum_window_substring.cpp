// problem: https://leetcode.cn/problems/minimum-window-substring/
// 示例 1:
// 输入: s = "ADOBECODEBANC", t = "ABC"
// 输出: "BANC"
// 解释: 最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'.
// 
// 示例 2:
// 输入: s = "a", t = "a"
// 输出: "a"
// 解释: 整个字符串 s 是最小覆盖子串.
// 
// 示例 3:
// 输入: s = "a", t = "aa"
// 输出: ""
// 解释: t 中两个字符 'a' 均应包含在 s 的子串中.
// 因此没有符合条件的子字符串，返回空字符串.
#include <string>
#include <unordered_map>
using namespace std;

class Solution {
public:
    string minWindow(string s, string t) {
        std::transform(s.begin(), s.end(), s.begin(), [](unsigned char ch) { return std::tolower(ch); });
        std::transform(t.begin(), t.end(), t.begin(), [](unsigned char ch) { return std::tolower(ch); });
        unordered_map<char, int> needs, umap;
        for (auto ch : t) needs[ch]++;
        int left = 0, right = 0;
        int valid = 0;
        int start = 0, min_len = INT_MAX;

        while (right < s.size()) {
            char rc = s[right];
            right ++;
            if (needs.count(rc)) {
                umap[rc] ++;
                if (umap[rc] == needs[rc]) valid ++;
            }
            while (valid == needs.size()) {
                if (min_len > right - left) {
                    start = left;
                    min_len = right - left;
                }
                char lc = s[left];
                left ++;
                if (needs.count(lc)) {
                    if (needs[lc] == umap[lc]) valid --;
                    umap[lc] --;
                }
            }
        }

        return min_len == INT_MAX ? "" : s.substr(start, min_len);
    }
};

int main()
{
    return 0;
}