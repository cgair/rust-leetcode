// 问题描述:
// 给定两个单词 word1 和 word2, 返回使得 word1 和  word2 相同所需的最小步数.
// 每步 可以删除任意一个字符串中的一个字符.
// 
// problem: https://leetcode.cn/problems/delete-operation-for-two-strings/
#include <string>
#include <vector>
#include <iostream>

int min_distance(std::string word1, std::string word2) {
    int n1 = word1.length(), n2 = word2.length();
    // dp[i][j] word1[0..=i] 和 word2[0..=j] 相同所需的最小步数.
    std::vector<std::vector<int>> dp(n1 + 1, std::vector<int>(n2 + 1, 0));
    for (int i = 0; i <= n1; ++i) dp[i][0] = i;
    for (int i = 0; i <= n2; ++i) dp[0][i] = i;

    for (int i = 1; i <= n1; ++i) {
        for (int j = 1; j <= n2; ++j) {
            if (word1[i - 1] == word2[j - 1]) dp[i][j] = dp[i - 1][j - 1];
            else dp[i][j] = std::min({dp[i - 1][j - 1] + 2, dp[i - 1][j] + 1, dp[i][j - 1] + 1});
        }
    }

    return dp[n1][n2];
}

int main()
{
    int t;
    std::cin >> t;

    std::vector<std::string> strs;
    while (t --) {
        std::string input;
        while (std::cin >> input) {
            strs.push_back(input);
            if (getchar() == '\n') {
                std::string word1 = strs[0], word2 = strs[1];
                std::cout << "min distance of "
                          << word1 << " and " << word2 << " is "
                          << min_distance(word1, word2)
                          << std::endl;
                strs.clear();
            }
        }
    }

    return 0;
}