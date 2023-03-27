/**
 * [62] Unique Paths
 *
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
 *
 * The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
 *
 * How many possible unique paths are there?
 *
 * ![Above is a 7 x 3 grid. How many possible unique paths are there?](https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png)
 *
 * Note: m and n will be at most 100.
 *
 * Example 1:
 *
 *
 * Input: m = 3, n = 2
 * Output: 3
 * Explanation:
 * From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
 * 1. Right -> Right -> Down
 * 2. Right -> Down -> Right
 * 3. Down -> Right -> Right
 *
 *
 * Example 2:
 *
 *
 * Input: m = 7, n = 3
 * Output: 28
 *
 */
pub struct Solution;

// problem: https://leetcode.cn/problems/unique-paths/

// submission codes start here

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp_table = vec![vec![0; m as usize];n as usize];

        for i in 0..m {
            dp_table[0][i as usize] = 1;
        }
        for i in 0..n {
            dp_table[i as usize][0] = 1;
        }

        for i in 1..n as usize {
            for j in 1..m as usize {
                dp_table[i][j] = dp_table[i - 1][j] + dp_table[i][j - 1];
            }
        }

        dp_table[n as usize - 1][m as usize - 1]
    }

    pub fn unique_paths_opt(m: i32, n: i32) -> i32 {
        let mut dp_table = vec![vec![0; m as usize];2 as usize];

        for i in 0..m {
            dp_table[0][i as usize] = 1;
        }
        for i in 0..2 {
            dp_table[i as usize][0] = 1;
        }

        for i in 1..n as usize {
            for j in 1..m as usize {
                let idx = i & 1;

                dp_table[idx][j] = dp_table[idx^1][j] + dp_table[idx][j - 1];
            }
        }

        dp_table[n as usize & 1 ^ 1][m as usize - 1]
    }

    // TODO(cgair) 压缩成一维
    pub fn unique_paths_opt2(m: i32, n: i32) -> i32 {
        let mut dp_table = vec![0;m as usize];


        todo!()
    }

    // TODO(cgair) 数论方法
    // its high school math: C(r,n) = n! / r!(n-r)! ...are you fxxking kidding me?
    // ...high school math will attempt to i32 overflow, we have to do it clever
    pub fn unique_paths2(m: i32, n: i32) -> i32 {

        todo!()
    }

}


#[test]
fn test_62() {
    assert_eq!(Solution::unique_paths(7, 3), 28);
    assert_eq!(Solution::unique_paths(3, 7), 28);
    assert_eq!(Solution::unique_paths(1, 1), 1);
    assert_eq!(Solution::unique_paths(2, 2), 2);
    assert_eq!(Solution::unique_paths(36, 7), 4496388);


    assert_eq!(Solution::unique_paths_opt(7, 3), 28);
    assert_eq!(Solution::unique_paths_opt(3, 7), 28);
    assert_eq!(Solution::unique_paths_opt(1, 1), 1);
    assert_eq!(Solution::unique_paths_opt(2, 2), 2);
    assert_eq!(Solution::unique_paths_opt(36, 7), 4496388);
}