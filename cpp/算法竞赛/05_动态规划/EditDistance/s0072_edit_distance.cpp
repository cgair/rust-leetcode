/**
 * [72] Edit Distance
 *
 * Given two words word1 and word2, find the minimum number of operations required to convert word1 to word2.
 *
 * You have the following 3 operations permitted on a word:
 *
 * 
 * 	Insert a character
 * 	Delete a character
 * 	Replace a character
 * 
 *
 * Example 1:
 *
 *
 * Input: word1 = "horse", word2 = "ros"
 * Output: 3
 * Explanation:
 * horse -> rorse (replace 'h' with 'r')
 * rorse -> rose (remove 'r')
 * rose -> ros (remove 'e')
 *
 *
 * Example 2:
 *
 *
 * Input: word1 = "intention", word2 = "execution"
 * Output: 5
 * Explanation:
 * intention -> inention (remove 't')
 * inention -> enention (replace 'i' with 'e')
 * enention -> exention (replace 'n' with 'x')
 * exention -> exection (replace 'n' with 'c')
 * exection -> execution (insert 'u')
 *
 */

// problem: https://leetcode.cn/problems/edit-distance/
#include <string>
#include <vector>

using namespace std;
// submission codes start here

class Solution {
public:
    int minDistance(string word1, string word2) {
        // dp[i][j] 表示
        // 以下标 i - 1 为结尾的字符串 word1 和以下标 j - 1 为结尾的字符串 word2
        // 最少编辑距离为dp[i][j]
        int n1 = word1.length(), n2 = word2.length();
        vector<vector<int>> dp(n1 + 1, vector<int>(n2 + 1, 0));
        for (int i = 0; i <= n1; ++i) dp[i][0] = i;
        for (int i = 0; i <= n2; ++i) dp[0][i] = i;
        
        for (int i = 1; i <= n1; ++i) {
            for (int j = 1; j <= n2; ++j) {
                if (word1[i - 1] == word2[j - 1]) dp[i][j] = dp[i - 1][j - 1];
                else {
                    dp[i][j] = min({
                        // word1 删除一个元素
                        dp[i - 1][j] + 1,
                        // word2 删除一个元素
                        dp[i][j - 1] + 1,
                        // word2 添加一个元素, 
                        // 相当于 word1 删除一个元素
                        // 反之亦然
                        // 替换元素
                        dp[i][j] = dp[i - 1][j - 1] + 1
                    });
                }
            }
        }
        
        return dp[n1][n2];
    }
};