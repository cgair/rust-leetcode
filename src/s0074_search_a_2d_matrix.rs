/**
 * [74] Search a 2D Matrix
 *
 * You are given an m x n integer matrix matrix with the following two properties:
 * Each row is sorted in non-decreasing order.
 * The first integer of each row is greater than the last integer of the previous row.
 * 
 * Given an integer target, return true if target is in matrix or false otherwise.
 * You must write a solution in O(log(m * n)) time complexity.
 * 
 *
 * Example1:
 *
 * 
 * Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
 * Output: true
 * 
 * Example2:
 *
 * 
 * Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
 * Output: false
 * 
 * 
 */

pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let nums = matrix.iter().flatten().collect::<Vec<_>>();
        let (mut left, mut right) = (0, nums.len());
        
        while left < right {
            let mid = left + (right - left) / 2;

            if *nums[mid] < target {
                left = mid + 1;
            } else if *nums[mid] > target {
                right = mid;
            } else if *nums[mid] == target {
                return true;
            }
        }

        false
        // TODO(cgair) 不用 flatten:
        // 已知二维数组的行数 `m` 和列数 `n`, 二维坐标 `(i, j)` 
        // 可以映射成一维: `index = i * n + j`, 
        // 反之亦可得 `i = index / n, j = index % n`.
    }
}


#[test]
fn test_74() {
    assert_eq!(
        Solution::search_matrix(vec![vec![1]], 0),
        false
    );
    assert_eq!(
        Solution::search_matrix(vec![vec![1, 1]], 0),
        false
    );
    assert_eq!(
        Solution::search_matrix(vec![vec![1, 1]], 2),
        false
    );
}