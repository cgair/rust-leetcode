/**
 * [392] Is Subsequence
 * Given two strings s and t, 
 * return true if s is a subsequence of t, 
 * or false otherwise.
 * 
 * A subsequence of a string is a new string that is formed from 
 * the original string by deleting some (can be none) of the characters 
 * without disturbing the relative positions of the remaining characters. 
 * (i.e., "ace" is a subsequence of "abcde" while "aec" is not).
*/
// problem: https://leetcode.cn/problems/is-subsequence/
#include <string>
#include <vector>
using namespace std;
// submission codes start here
class Solution {
public:
    bool isSubsequence(string s, string t) {
        int ns = s.length(), nt = t.length();
        int ps = 0, pt = 0;
        while (ps < ns && pt < nt) {
            if (s[ps] == t[pt]) {
                ps ++;
            }
            pt ++;
        }

        return ps == ns;
    }

    // 动态规划
    bool dynamicProgramming(string s, string t) {
        int ns = s.length(), nt = t.length();
        vector<vector<int>> dp(ns + 1, vector<int>(nt + 1, 0));
        for (int i = 1; i <= ns; ++i) {
            for (int j = 1; j <= nt; ++j) {
                if (s[i-1] == t[j-1]) dp[i][j] = dp[i - 1][j - 1] + 1;
                else dp[i][j] = dp[i][j - 1];
            }
        }

        return dp[ns][nt] == ns;
    }
};