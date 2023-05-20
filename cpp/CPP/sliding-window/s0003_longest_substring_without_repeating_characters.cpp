// problem: https://leetcode.cn/problems/longest-substring-without-repeating-characters
// 示例 1:
// 输入: s = "abcabcbb"
// 输出: 3 
// 解释: 因为无重复字符的最长子串是 "abc", 所以其长度为 3.
// 
// 示例 2:
// 输入: s = "bbbbb"
// 输出: 1
// 解释: 因为无重复字符的最长子串是 "b", 所以其长度为 1.
// 
// 示例 3:
// 输入: s = "pwwkew"
// 输出: 3
// 解释: 因为无重复字符的最长子串是 "wke", 所以其长度为 3.
//      请注意, 你的答案必须是子串的长度, "pwke" 是一个子序列, 不是子串.
#include <string>
#include <unordered_set>
using namespace std;

class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        if (s.empty()) return 0;
        int l = s.size();
        int left = 0, right = 0;
        unordered_set<char> window;
        int max_len = INT_MIN;

        while(right < l) {
            char rc = s[right];
            right ++;
            while (window.count(rc))
            {
                char lc = s[left];
                left ++;
                window.erase(lc);
            }
            window.emplace(rc);
            max_len = max(max_len, right - left);
        }

        return max_len;
    }
};

int main() 
{
    Solution s;
    int ret = s.lengthOfLongestSubstring("abcabcbb");
    assert(ret == 3);

    return 0;
}