// 最长公共子序列 (Longest Common Subsequence, LCS)
// problem: https://leetcode.cn/problems/longest-common-subsequence/
#include <string>
#include <iostream>
#include <vector>
using namespace std;


class Solution {
public:
    int longestCommonSubsequence(string text1, string text2) {
        // dp[i][j]: text1 的前 i 个元素的序列和 text2 的前 j 个元素的序列
        // 的最长公共子序列的**长度**
        int n1 = text1.length(), n2 = text2.length();
        vector<vector<int> > dp(n1 + 1, vector<int>(n2 + 1, 0));
        
        for (int i = 1; i <= n1; ++i) {
            for (int j = 1; j <= n2; ++j) {
                if (text1[i-1] == text2[j-1]) {
                    dp[i][j] = dp[i-1][j-1] + 1;
                } else {
                    // we have to consider two possibilities: 
                    // either the i-th character of text1 contributes to the LCS 
                    // while the j-th character of text2 does not, 
                    // or vice versa. 
                    dp[i][j] = max(dp[i][j-1], dp[i-1][j]);
                }
            }
        }

        return dp[n1][n2];
    }
};


int main()
{

    return 0;
}