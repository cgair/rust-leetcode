/**
 * [70] Climbing Stairs
 *
 * You are climbing a stair case. It takes n steps to reach to the top.
 *
 * Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
 *
 * Note: Given n will be a positive integer.
 *
 * Example 1:
 *
 *
 * Input: 2
 * Output: 2
 * Explanation: There are two ways to climb to the top.
 * 1. 1 step + 1 step
 * 2. 2 steps
 *
 *
 * Example 2:
 *
 *
 * Input: 3
 * Output: 3
 * Explanation: There are three ways to climb to the top.
 * 1. 1 step + 1 step + 1 step
 * 2. 1 step + 2 steps
 * 3. 2 steps + 1 step
 *
 *
 */
pub struct Solution;

// problem: https://leetcode.cn/problems/climbing-stairs/

// submission codes start here

// Bottom-up DP
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 0 { return 0; }
        let mut dp_table = vec![0;n as usize];
        // base case
        dp_table[0] = 1;
        dp_table[1] = 2;

        for idx in 2..n {
            // 首先是 dp[i - 1], 上 i-1 层楼梯, 有 dp[i - 1] 种方法, 那么再一步跳一个台阶不就是 dp[i] 啦!
            // 然后是 dp[i - 2], 上 i-2 层楼梯, 有 dp[i - 2] 种方法, 那么再一步跳两个台阶不就是 dp[i] 啦!
            // 那么 dp[i] 就是 dp[i - 1] 与 dp[i - 2] 之和!
            dp_table[idx as usize] = dp_table[idx as usize - 1] + dp_table[idx as usize - 2];
        }

        dp_table[n as usize - 1]
    }
}

#[test]
fn test_70() {
    assert_eq!(Solution::climb_stairs(3), 3);
    assert_eq!(Solution::climb_stairs(4), 5);
    assert_eq!(Solution::climb_stairs(5), 8);
}