// 最长递增子序列 (Longest Increasing Subsequence, LIS)
// problem: https://leetcode.cn/problems/longest-increasing-subsequence/
#include <vector>
#include <iostream>
using namespace std;

class Solution {
public:
    int lengthOfLIS(vector<int>& nums) {
        int n = nums.size();
        // 起始大小至少都是是 1.
        vector<int> dp(n, 1);
        int ret = 0;
        // dp[i] 表示以第 i 个数结尾的最长递增子序列的长度 (i 是 index)
        // 位置 i 的最长升序子序列等于 j 从 0 到 i-1 各个位置的最长升序子序列 + 1 的最大值
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < i; ++j) {
                if (nums[j] < nums[i])  dp[i] = max(dp[j] + 1, dp[i]);
            }
            if (dp[i] > ret) ret = dp[i];
        }

        return ret;
    }
};

int main()
{
    Solution s;
    vector<int> case1 = {0};
    cout << s.lengthOfLIS(case1) << endl;
    
    vector<int> case2 = {10,9,2,5,3,7,101,18};
    cout << s.lengthOfLIS(case2) << endl; // 4

    return 0;
}