/**
 * [583] Delete Operation for Two Strings
 * Given two strings word1 and word2, 
 * return the minimum number of steps required to make word1 and word2 the same.
 * 
 * In one step, you can delete exactly one character in either string.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: word1 = "sea", word2 = "eat"
 * Output: 2
 * Explanation: You need one step to make "sea" to "ea" and another step to make "eat" to "ea".
 * 
 * 
 * Example 2:
 * Input: word1 = "leetcode", word2 = "etco"
 * Output: 4
 * 
 * 
 * Constraints:
 * word1 and word2 consist of only lowercase English letters.
*/
// problem: https://leetcode.cn/problems/delete-operation-for-two-strings/
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

// submission codes start here
class Solution {
public:
    int minDistance(string word1, string word2) {
        // 题目说每步可以删除任意一个字符串中的一个字符.
        // 
        // dp[i][j]: 以 i - 1 为结尾的字符串 word1 和以 j - 1 位结尾的字符串 word2, 
        // 想要达到相等, 所需要删除元素的最少次数.
        int n1 = word1.length(), n2 = word2.length();
        vector<vector<int> > dp(n1 + 1, vector<int>(n2 + 1, 0));
        for (int i = 0; i <= n1; ++i) dp[i][0] = i;
        for (int i = 0; i <= n2; ++i) dp[0][i] = i;
        
        for (int i = 1; i <= n1; ++i) {
            for (int j = 1; j <= n2; ++j) {
                if (word1[i - 1] == word2[j - 1]) dp[i][j] = dp[i - 1][j - 1];
                else {
                    // 有三种情况:
                    // 情况一: 删 word1[i - 1], 最少操作次数为 dp[i - 1][j] + 1
                    // 情况二: 删 word2[j - 1], 最少操作次数为 dp[i][j - 1] + 1
                    // 情况三: 同时删 word1[i - 1] 和 word2[j - 1], 操作的最少次数为 dp[i - 1][j - 1] + 2 
                    // dp[i][j] = min(min(dp[i - 1][j] + 1, dp[i][j - 1] + 1), dp[i - 1][j - 1] + 2); 
                    dp[i][j] = min({dp[i - 1][j - 1] + 2, dp[i - 1][j] + 1, dp[i][j - 1] + 1});
                }

            }
        }

        return dp[n1][n2];
    }
};