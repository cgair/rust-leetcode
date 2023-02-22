/**
 * [48] Rotate Image
 *
 * You are given an n x n 2D matrix representing an image.
 *
 * Rotate the image by 90 degrees (clockwise).
 *
 * Note:
 *
 * You have to rotate the image in-place, which means you have to modify the input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.
 *
 * Example 1:
 *
 *
 * Given input matrix =
 * [
 *   [1,2,3],
 *   [4,5,6],
 *   [7,8,9]
 * ],
 *
 * rotate the input matrix in-place such that it becomes:
 * [
 *   [7,4,1],
 *   [8,5,2],
 *   [9,6,3]
 * ]
 *
 *
 * Example 2:
 *
 *
 * Given input matrix =
 * [
 *   [ 5, 1, 9,11],
 *   [ 2, 4, 8,10],
 *   [13, 3, 6, 7],
 *   [15,14,12,16]
 * ],
 *
 * rotate the input matrix in-place such that it becomes:
 * [
 *   [15,13, 2, 5],
 *   [14, 3, 4, 1],
 *   [12, 6, 8, 9],
 *   [16, 7,10,11]
 * ]
 *
 *
 */

// problem: https://leetcode.cn/problems/rotate-image/

// submission codes start here

pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let row = matrix.len();
        let col = matrix[0].len();

        for r in 0..row {
            for c in r..col {
                let tmp = matrix[r][c];
                matrix[r][c] = matrix[c][r];
                matrix[c][r] = tmp;
            }
        }

        for row in matrix.iter_mut() {
            row.reverse();
        }
    }

    fn reverse(arr: &mut Vec<i32>) {
        let len = arr.len();
        let (mut lo, mut hi) = (0usize, len - 1);
        while lo < hi {
            arr.swap(lo, hi);
            lo += 1;
            hi -= 1;
        }
    }
}

#[test]
fn test_48() {
    let mut matrix = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut matrix);
    assert_eq!(
        matrix,
        vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11]
        ]
    );
}