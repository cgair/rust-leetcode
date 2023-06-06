/**
 * [115] Distinct Subsequences
 *
 * Given a string S and a string T, count the number of distinct subsequences of S which equals T.
 *
 * A subsequence of a string is a new string which is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, "ACE" is a subsequence of "ABCDE" while "AEC" is not).
 *
 * Example 1:
 *
 *
 * Input: S = "rabbbit", T = "rabbit"
 * Output: 3
 * Explanation:
 *
 * As shown below, there are 3 ways you can generate "rabbit" from S.
 * (The caret symbol ^ means the chosen letters)
 *
 * rabbbit
 * ^^^^ ^^
 * rabbbit
 * ^^ ^^^^
 * rabbbit
 * ^^^ ^^^
 *
 *
 * Example 2:
 *
 *
 * Input: S = "babgbag", T = "bag"
 * Output: 5
 * Explanation:
 *
 * As shown below, there are 5 ways you can generate "bag" from S.
 * (The caret symbol ^ means the chosen letters)
 *
 * babgbag
 * ^^ ^
 * babgbag
 * ^^    ^
 * babgbag
 * ^    ^^
 * babgbag
 *   ^  ^^
 * babgbag
 *     ^^^
 *
 */

// problem: https://leetcode.cn/problems/distinct-subsequences/
#include <string>
#include <vector>
#include <iostream>

using namespace std;
// submission codes start here
class Solution {
public:
    int numDistinct(string s, string t) {
        int ns = s.length(), nt = t.length();
        // dp[i][j]: 以 i - 1 为结尾的 s 子序列中出现以 j - 1 为结尾的 t 的个数为 dp[i][j].
        vector<vector<int> > dp(ns + 1, vector<int>(nt + 1, 0));
        for (int i = 0; i <= ns; ++i) dp[i][0] = 1;

        for (int i = 1; i <= ns; ++i) {
            for (int j = 1; j <= nt; ++j) {
                // 用 s[i - 1] 匹配, 个数为dp[i - 1][j - 1]
                // 不用 s[i - 1] 匹配, 个数为dp[i - 1][j]
                if (s[i - 1] == t[j - 1]) dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
                else dp[i][j] = dp[i - 1][j];
            }
        }

        for (auto &inner : dp) {
            cout << "[ ";
            for (auto x : inner) {
                cout << x << " ";
            }
            cout << "]\n";
        }
        
        return dp[ns][nt];
    }
};

int main()
{
    Solution s;
    cout << s.numDistinct("rabbbit", "rabbit") << endl;

    return 0;
}