/**
 * [509] Fibonacci Number
 *
 * The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1. That is,
 *
 *
 * F(0) = 0,   F(1) = 1
 * F(N) = F(N - 1) + F(N - 2), for N > 1.
 *
 *
 * Given N, calculate F(N).
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: 2
 * Output: 1
 * Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
 *
 *
 * Example 2:
 *
 *
 * Input: 3
 * Output: 2
 * Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
 *
 *
 * Example 3:
 *
 *
 * Input: 4
 * Output: 3
 * Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.
 *
 *
 *  
 *
 * Note:
 *
 * 0 <= N <= 30.
 *
 */
pub struct Solution;

// problem: https://leetcode.cn/problems/fibonacci-number/

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn fib_ugly(n: i32) -> i32 {
        if n == 0 { return 0; }
        if n == 1 { return 1; }
        
        Solution::fib_ugly(n - 1) + Solution::fib_ugly(n - 2)
    }

    pub fn fib(n: i32) -> i32 {
        let mut memo = HashMap::new();

        Solution::recursion(n, &mut memo)
    }

    fn recursion(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n == 0 { return 0; }
        if n == 1 { return 1; }
        if memo.contains_key(&n) { return *memo.get(&n).unwrap(); }
        
        let ret = Solution::recursion(n - 1, memo) + Solution::recursion(n - 2, memo);
        memo.insert(n, ret);

        ret
    }
}

#[test]
fn test_1013() {
    assert_eq!(Solution::fib_ugly(2), 1);
    assert_eq!(Solution::fib_ugly(3), 2);
    assert_eq!(Solution::fib_ugly(4), 3);

    assert_eq!(Solution::fib(2), 1);
    assert_eq!(Solution::fib(3), 2);
    assert_eq!(Solution::fib(4), 3);
}