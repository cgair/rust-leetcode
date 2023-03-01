/**
 * [51] N-Queens
 *
 * The n-queens puzzle is the problem of placing n queens on an n*n chessboard such that no two queens attack each other.
 * 
 * PS: 皇后可以攻击同一行, 同一列, 左上, 左下, 右上, 右下四个方向的任意单位.
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/10/12/8-queens.png" style="width: 258px; height: 276px;" />
 *
 * Given an integer n, return all distinct solutions to the n-queens puzzle.
 *
 * Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space respectively.
 *
 * Example:
 *
 *
 * Input: 4
 * Output: [
 *  [".Q..",  // Solution 1
 *   "...Q",
 *   "Q...",
 *   "..Q."],
 *
 *  ["..Q.",  // Solution 2
 *   "Q...",
 *   "...Q",
 *   ".Q.."]
 * ]
 * Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above.
 *
 *
 */
// problem: https://leetcode.cn/problems/n-queens/

// submission codes start here
pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        assert!(n >= 0, "n >= 0");
        let mut chessboard = vec![vec![String::from("."); n as usize]; n as usize];
        let mut ret = Vec::new();
            
        Solution:: backtrack(&mut chessboard, &mut ret, n as usize, 0);

        ret
    }

    fn backtrack(chessboard: &mut Vec<Vec<String>>, ret: &mut Vec<Vec<String>>, n: usize, row: usize) {
        if row == n {
            let mut concat = Vec::new();
            for row in chessboard.iter() {
                let mut s = String::new();
                for ch in row {
                    s.push_str(ch);
                }
                concat.push(s);
            }

            ret.push(concat);
            return;
        }
        
        for col in 0..n {
            if !Solution::can_put(&chessboard, row, col) {
                continue;
            }
            chessboard[row][col] = "Q".to_string();
            Solution::backtrack(chessboard, ret, n, row + 1);
            chessboard[row][col] = ".".to_string();
        }
    }

    /// Whether a Queue can be put on chessboard[row][col]?
    /// 
    fn can_put(chessboard: &Vec<Vec<String>>, row: usize, col: usize) -> bool {
        if row > 0 {
            // Checks if a column has a conflict of queens
            for i in (0..row).rev() {
                if chessboard[i][col] == "Q" { return false; }
            }
        }
        
        // Check the upper left for a conflict of queens
        let (mut r, mut c) = (row, col);
        while r > 0 && c > 0 {
            if chessboard[r - 1][c - 1] == "Q" { return false; }
            r -= 1;
            c -= 1
        }
        
        // Check the upper right for a conflict of queens
        let (mut r, mut c) = (row, col);
        while r > 0 && c < chessboard.len() - 1 {
            if chessboard[r - 1][c + 1] == "Q" { return false; }
            r -= 1;
            c += 1
        }

        true
    }
}


#[test]
fn test_51() {
    let values = vec![0, 1, 2, 3, 4, 5, 6];
    for i in (0..7).rev() {
        println!("{}", values[i]);
    }

    assert_eq!(
        Solution::solve_n_queens(4),
        vec![
            vec![".Q..", "...Q", "Q...", "..Q."],
            vec!["..Q.", "Q...", "...Q", ".Q.."]
        ]
    );
    assert_eq!(
        Solution::solve_n_queens(5),
        vec![
            ["Q....","..Q..","....Q",".Q...","...Q."],
            ["Q....","...Q.",".Q...","....Q","..Q.."],
            [".Q...","...Q.","Q....","..Q..","....Q"],
            [".Q...","....Q","..Q..","Q....","...Q."],
            ["..Q..","Q....","...Q.",".Q...","....Q"],
            ["..Q..","....Q",".Q...","...Q.","Q...."],
            ["...Q.","Q....","..Q..","....Q",".Q..."],
            ["...Q.",".Q...","....Q","..Q..","Q...."],
            ["....Q",".Q...","...Q.","Q....","..Q.."],
            ["....Q","..Q..","Q....","...Q.",".Q..."]]
    );

    assert_eq!(Solution::solve_n_queens(8).len(), 92);
}