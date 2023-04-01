/**
 * [50] Pow(x, n)
 *
 * Implement pow(x, n), which calculates x raised to the power n (i.e., x^n).
 *
 * Example 1:
 *
 *
 * Input: 2.00000, 10
 * Output: 1024.00000
 *
 *
 * Example 2:
 *
 *
 * Input: 2.10000, 3
 * Output: 9.26100
 *
 *
 * Example 3:
 *
 *
 * Input: 2.00000, -2
 * Output: 0.25000
 * Explanation: 2^-2 = 1/2^2 = 1/4 = 0.25
 *
 *
 * Note:
 *
 *
 * 	-100.0 < x < 100.0
 * 	n is a 32-bit signed integer, within the range [-2^31, 2^31 - 1]
 *
 *
 */
pub struct Solution;

// problem: https://leetcode.cn/problems/powx-n/

// submission codes start here

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 { return 1.; }
        let (mut n, mut x)  = (n, x);
        let mut flag = false;
        if n < 0 {
            n = -n;
            flag = true;
        }
        // 非递归式在实践过程中的速度是比递归式更快的(因为递归会花费一定的开销)
        let mut ret = 1.;
        while n > 0 {
            if n & 1 == 1 { ret = ret * x; }

            x = x * x;
            n = n >> 1;
        }
        
        return if flag { 1. / ret} else { ret };
    }

    pub fn recursive(x: f64, n: i32) -> f64 {
        if n == 0 { return 1.; }
        let mut ret = Solution::recursive(x, n / 2);
        if n % 2 == 0 {
            return ret * ret;
        } else {
            return x * ret * ret;
        }

        ret
    }
}


#[test]
fn test_50() {
    assert_eq!(Solution::my_pow(2.0, 4), 16.0);
    assert_eq!(Solution::my_pow(2.0, 5), 32.0);
    assert_eq!(Solution::my_pow(2.0, 1), 2.0);
    assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
    assert_eq!(Solution::my_pow(2.0, -1), 0.5);
    assert_eq!(Solution::my_pow(2.0, -2), 0.25);

    assert_eq!(Solution::recursive(2.0, 4), 16.0);
    assert_eq!(Solution::recursive(2.0, 5), 32.0);
    assert_eq!(Solution::recursive(2.0, 1), 2.0);
    assert_eq!(Solution::recursive(2.0, 10), 1024.0);
}