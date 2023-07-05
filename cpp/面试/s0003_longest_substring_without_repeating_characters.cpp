// 3. 无重复字符的最长子串
// problem: https://leetcode.cn/problems/longest-substring-without-repeating-characters/
#include <string>
#include <unordered_set>
using namespace std;


class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        if (s.empty()) return 0;
        unordered_set<char> uset;
        int ret = 0;

        int left = 0, right = 0;
        while (right < s.size()) {
            char rc = s[right];
            right ++;
            while (uset.count(rc)) {
                char lc = s[left];
                left ++;
                uset.erase(lc);
            }
            uset.emplace(rc);
            ret = max(ret, right - left);
        }

        return ret;
    }
};

