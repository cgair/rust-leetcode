/**
 * [54] Spiral Matrix
 *
 * Given a matrix of m x n elements (m rows, n columns), return all elements of the matrix in spiral order.
 *
 * Example 1:
 *
 *
 * Input:
 * [
 *  [ 1, 2, 3 ],
 *  [ 4, 5, 6 ],
 *  [ 7, 8, 9 ]
 * ]
 * Output: [1,2,3,6,9,8,7,4,5]
 *
 *
 * Example 2:
 *
 * Input:
 * [
 *   [1, 2, 3, 4],
 *   [ 5, 6, 7, 8],
 *   [9,10,11,12]
 * ]
 * Output: [1,2,3,4,8,12,11,10,9,5,6,7]
 *
 */

// problem: https://leetcode.cn/problems/spiral-matrix/

// submission codes start here
pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let row = matrix.len();
        assert!(row >= 1, "m >= 1");
        let col = matrix[0].len();

        let mut ret = Vec::with_capacity(row * col);

        /*
        let round = (row + 2 - 1) / 2;

        for rd in 0..round {
            if ret.len() == row * col { break; }
            // ->
            for idx in rd..col - rd {
                ret.push(matrix[rd][idx]);
            }
            println!("LEFT to RIGHT: {:?}", ret);
            // |
            // V
            for idx in (rd + 1)..(row - rd - 1){
                ret.push(matrix[idx][col - 1 - rd]);
            }
            println!("   UP to DOWN: {:?}", ret);
            // <-
            for idx in (rd..col - rd).rev() {
                ret.push(matrix[row - 1 - rd][idx]);
            }
            println!("RIGHT to LEFT: {:?}", ret);
            // ^
            // |
            for idx in ((rd + 1)..(row - rd - 1)).rev(){
                ret.push(matrix[idx][rd]);
            }
            println!("   DOWN to UP: {:?}", ret);
        }
        */

        // 解题的核心思路是按照右、下、左、上的顺序遍历数组, 并使用四个变量圈定未遍历元素的边界:
        let (mut upper_bound, mut lower_bound, mut left_bound, mut right_bound) = (0i32, (row - 1) as i32, 0i32, (col - 1) as i32);

        while ret.len() < row * col {
            // ->
            if upper_bound <= lower_bound {
                for idx in left_bound..=right_bound {
                    ret.push(matrix[upper_bound as usize][idx as usize]);
                }
                upper_bound += 1;
            }
            // |
            // V
            if left_bound <= right_bound {
                for idx in upper_bound..=lower_bound {
                    ret.push(matrix[idx as usize][right_bound as usize]);
                }
                right_bound -= 1;
            }
            // <-
            if upper_bound <= lower_bound {
                for idx in (left_bound..=right_bound).rev() {
                    ret.push(matrix[lower_bound as usize][idx as usize])
                }
                lower_bound -= 1;
            }
            // ^
            // |
            if left_bound <= right_bound {
                for idx in (upper_bound..=lower_bound).rev() {
                    ret.push(matrix[idx as usize][left_bound as usize]);
                }
                left_bound += 1;
            }
        }

        ret
    }
}

#[test]
fn test_54() {
    // for i in 1..0 {
    //     print!("{} ", i);
    // }
    // println!();
    // for i in -1..8 {
    //     print!("{} ", i);
    // }
    // println!("{}", -1i32 as usize);
    println!();
    assert_eq!(
        Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
    );
    assert_eq!(Solution::spiral_order(vec![vec![1, 2, 3]]), vec![1, 2, 3]);
    assert_eq!(
        Solution::spiral_order(vec![vec![1], vec![2], vec![3]]),
        vec![1, 2, 3]
    );
    assert_eq!(Solution::spiral_order(vec![vec![1]]), vec![1]);
    assert_eq!(
        Solution::spiral_order(vec![vec![1, 2], vec![4, 5]]),
        vec![1, 2, 5, 4]
    );
}