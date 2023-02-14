/**
 * [322] Coin Change
 *
 * You are given an integer array coins representing coins of different denominations 
 * and an integer amount representing a total amount of money.
 * Return the fewest number of coins that you need to make up that amount. 
 * If that amount of money cannot be made up by any combination of the coins, return -1.
 * 
 * You may assume that you have an infinite number of each kind of coin.
 *
 * Example1:
 *
 *
 * Input: coins = [1,2,5], amount = 11
 *
 * Output: 3
 * Explanation: 11 = 5 + 5 + 1
 * 
 * Example1:
 * 
 * 
 * Input: coins = [2], amount = 3
 * Output: -1
 */


pub struct Solution;
use std::cmp;

impl Solution {
    // Calculate the minimum number of coins needed to make a given amount.
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount < 0 { return -1; }
        // dp table: 
        // when the target amount is i, at least dp[i] coins are needed to make it up.
        let len = (amount + 1) as usize;
        let mut dp = Vec::with_capacity(len);

        // 初始化为 amount + 1
        // 如果初始化为 std::u32::MAX 1 + dp[i - coin] 会 overflow
        // 凑成 amount 最多需要 amount (全是 1 cny), 此时 amount + 1 相当于 MAX
        for i in 0..len {
            dp.push(amount + 1);
        }
        
        // base case 
        dp[0] = 0;
        for i in 0..len {
            for &coin in coins.iter() {
                if (i as i32).checked_sub(coin).unwrap() < 0 {
                    continue;
                }
                dp[i] = cmp::min(dp[i], 1 + dp[i - coin as usize]);
            }
        }
        
        return if dp[amount as usize] == amount + 1 { -1 } else { dp[amount as usize] };
    }
}

#[test]
fn test_322() {
    let coins = vec![1, 2, 5];
    let amount = 11;
    assert_eq!(
        Solution::coin_change(coins, amount),
        3
    )
}