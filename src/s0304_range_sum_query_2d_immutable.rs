/**
 * [304] Range Sum Query 2D - Immutable
 *
 * Given a 2D matrix matrix, find the sum of the elements inside the rectangle defined by its upper left corner (row1, col1) and lower right corner (row2, col2).
 *
 *
 * Example:
 *
 * Given matrix = [
 *   [3, 0, 1, 4, 2],
 *   [5, 6, 3, 2, 1],
 *   [1, 2, 0, 1, 5],
 *   [4, 1, 0, 1, 7],
 *   [1, 0, 3, 0, 5]
 * ]
 *
 * sumRegion(2, 1, 4, 3) -> 8
 * sumRegion(1, 1, 2, 2) -> 11
 * sumRegion(1, 2, 2, 4) -> 12
 *
 *
 *
 * Note:
 * 
 * You may assume that the matrix does not change.
 * There are many calls to sumRegion function.
 * You may assume that row1 &le; row2 and col1 &le; col2.
 * 
 *
 */

// problem: https://leetcode.cn/problems/range-sum-query-2d-immutable/
// submission codes start here

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

struct NumMatrix {
    inner: Vec<Vec<i32>>,
    pre_sum: Vec<Vec<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        assert!(!matrix.is_empty());

        let row = matrix.len();
        let col = matrix[0].len();

        let mut pre_sum = vec![vec![0; col]; row];

        pre_sum[0][0] = matrix[0][0];

        for i in 1..col {
            pre_sum[0][i] = pre_sum[0][i - 1] + matrix[0][i];
        }
        for j in 1..row {
            pre_sum[j][0] = pre_sum[j - 1][0] + matrix[j][0];
        }

        // for i in 1..row {
        //     for j in 0..col {
        //         let mut supplement = 0;
        //         for k in 0..=j {
        //             supplement = supplement + matrix[i][k];
        //         }
        //         pre_sum[i][j] = pre_sum[i - 1][j] + supplement;
        //     }
        // }

        for i in 1..row {
            for j in 1..col {
                pre_sum[i][j] = pre_sum[i - 1][j] + pre_sum[i][j - 1] + matrix[i][j]  -  pre_sum[i - 1][j - 1];
            }
        }

        /* 受前缀和数组影响严重
        for r in matrix.iter() {
            let mut aux = Vec::with_capacity(row);
            let mut sum = 0;
            for &v in r.iter() {
                sum = sum + v;
                aux.push(sum);
            }
            pre_sum.push(aux);
        }
        */

        Self {
            inner: matrix,
            pre_sum
        }
    }
    
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        assert!(row1 <= row2 && col1 <= col2, "row1 <= row2 && col1 <= col2");
        /* 
        let mut ret = 0;
        // 用一个嵌套 for 循环去遍历这个矩阵的话 sumRegion 函数的时间复杂度就高了
        for r in row1..=row2 {
            ret = ret + {
              self.pre_sum[r as usize][col2 as usize] - if col1 > 0 { self.pre_sum[r as usize][(col1 - 1) as usize] } 
              else { 0 }
            };
        }

        ret
        */
        
        if col1 == 0 && row1 == 0 { return self.pre_sum[row2 as usize][col2 as usize]; }
        else if col1 == 0 && row1 > 0 { return self.pre_sum[row2 as usize][col2 as usize] - self.pre_sum[(row1 - 1) as usize][col2 as usize]; }
        else if col1 > 0 && row1 == 0 { return self.pre_sum[row2 as usize][col2 as usize] - self.pre_sum[row2 as usize][(col1 - 1) as usize]; }
        else { return self.pre_sum[row2 as usize][col2 as usize] - self.pre_sum[row2 as usize][(col1 - 1) as usize] - self.pre_sum[(row1 - 1) as usize][col2 as usize] + self.pre_sum[(row1 - 1) as usize][(col1 - 1) as usize]; }        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_304() {
        let matrix = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);
        println!("{:?}", matrix.pre_sum);
        assert_eq!(
            matrix.pre_sum,
            vec![
                vec![3, 3, 4, 8, 10],
                vec![8, 14, 18, 24, 27],
                vec![9, 17, 21, 28, 36],
                vec![13, 22, 26, 34, 49],
                vec![14, 23, 30, 38, 58]
            ]
        );
        assert_eq!(matrix.sum_region(1, 1, 2, 2), 11);
        assert_eq!(matrix.sum_region(2, 1, 4, 3), 8);
        assert_eq!(matrix.sum_region(1, 2, 2, 4), 12);
    }
}