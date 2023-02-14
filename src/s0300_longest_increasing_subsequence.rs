/**
 * [300] Longest Increasing Subsequence
 *
 * Given an unsorted array of integers, find the length of longest increasing subsequence.
 *
 * Example:
 *
 *
 * Input: [10,9,2,5,3,7,101,18]
 * Output: 4
 * Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
 *
 * Note:
 *
 *
 * 	There may be more than one LIS combination, it is only necessary for you to return the length.
 * 	Your algorithm should run in O(n^2) complexity.
 *
 *
 * Follow up: Could you improve it to O(n log n) time complexity?
 *
 */
use std::cmp;
pub struct Solution;

// problem: https://leetcode.cn/problems/longest-increasing-subsequence/

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // dp table 定义是这样的: dp[i] 表示以 nums[i] 这个数结尾的最长递增子序列的长度.
        let mut dp = Vec::new();

        // base case: dp[i] 初始值为 1
        // 因为以 nums[i] 结尾的最长递增子序列起码要包含它自己
        for _ in 0..nums.len() {
            dp.push(1);
        }
        
        // OR
        // let mut dp = vec![1; nums.len()];

        for i in 1..nums.len() {
            // 寻找 nums[0..j-1] 中比 nums[i] 小的元素
            for j in 0..i {
                if nums[i] > nums[j] {
                    // 把 nums[i] 接在后面, 即可形成长度为 dp[j] + 1，
                    // 且以 nums[i] 为结尾的递增子序列
                    dp[i] = cmp::max(dp[j] + 1, dp[i]);
                }
            }
        }

        let mut ret = 0;
        for &d in dp.iter() {
            ret = cmp::max(ret, d);
        }

        ret        
    }
}


#[test]
fn test_300() {
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
}