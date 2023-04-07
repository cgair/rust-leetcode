/**
 * [130] Surrounded Regions
 *
 * Given a 2D board containing 'X' and 'O' (the letter O), capture all regions surrounded by 'X'.
 *
 * A region is captured by flipping all 'O's into 'X's in that surrounded region.
 *
 * Example:
 *
 *
 * X X X X
 * X O O X
 * X X O X
 * X O X X
 *
 *
 * After running your function, the board should be:
 *
 *
 * X X X X
 * X X X X
 * X X X X
 * X O X X
 *
 *
 * Explanation:
 *
 * Surrounded regions shouldn't be on the border, which means that any 'O' on the border of the board are not flipped to 'X'. 
 * Any 'O' that is not on the border and it is not connected to an 'O' on the border will be flipped to 'X'. 
 * Two cells are connected if they are adjacent cells connected horizontally or vertically.
 *
 */
pub struct Solution;

// problem: https://leetcode.cn/problems/surrounded-regions/

// submission codes start here

/*
从最外层开始, 基于为 'O' 的格子做 DFS, 将与边界连接的所有 'O' 标记为一个特殊 char, 最后将没有标记到的 'O' 全部标记为 'X'
*/
const FLAG: char = '#';
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (row, col) = (board.len(), board[0].len());
        for i in 0..col {
            if board[0][i] == 'O' {
                Solution::dfs(board, 0, i as i32, row, col, FLAG);
            }
            if board[row - 1][i] == 'O' {
                Solution::dfs(board, row as i32 - 1, i as i32, row, col, FLAG);
            }
        }

        for i in 0..row {
            if board[i][0] == 'O' {
                Solution::dfs(board, i as i32, 0, row, col, FLAG);
            }
            if board[i][col - 1] == 'O' {
                Solution::dfs(board, i as i32, col as i32 - 1, row, col, FLAG);
            }
        }

        for i in 0..row {
            for j in 0..col {
                if board[i][j] == 'O' {
                    Solution::dfs(board, i as i32, j as i32, row, col, 'X');
                }
            }
        }

        for i in 0..row {
            for j in 0..col {
                if board[i][j] == FLAG { board[i][j] = 'O'; }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, i: i32, j: i32, row: usize, col: usize, substitution: char) {
        if i < 0 || j < 0 || i >= row as i32 || j >= col as i32 { return; }
        if board[i as usize][j as usize] == 'X' || board[i as usize][j as usize] == FLAG { return; }

        board[i as usize][j as usize] = substitution;

        Solution::dfs(board, i - 1, j, row, col, substitution);
        Solution::dfs(board, i + 1, j, row, col, substitution);
        Solution::dfs(board, i, j - 1, row, col, substitution);
        Solution::dfs(board, i, j + 1, row, col, substitution);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_130() {
        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['O', 'X', 'O', 'X'],
            vec!['O', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['O', 'X', 'X', 'X'],
                vec!['O', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec!['X', 'X', 'X', 'X', 'O', 'X'],
            vec!['O', 'X', 'X', 'O', 'O', 'X'],
            vec!['X', 'O', 'X', 'O', 'O', 'O'],
            vec!['X', 'O', 'O', 'O', 'X', 'O'],
            vec!['O', 'O', 'X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec!['X', 'X', 'X', 'X', 'O', 'X'],
                vec!['O', 'X', 'X', 'O', 'O', 'X'],
                vec!['X', 'O', 'X', 'O', 'O', 'O'],
                vec!['X', 'O', 'O', 'O', 'X', 'O'],
                vec!['O', 'O', 'X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'O', 'X', 'X'],
            ]
        );

        let mut matrix = vec![
            vec![
                'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'O', 'X', 'O', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
            vec![
                'X', 'X', 'X', 'X', 'X', 'O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                'X', 'X', 'X', 'X',
            ],
        ];
        Solution::solve(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![
                    'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'O', 'O', 'X', 'O', 'X', 'O', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'X', 'O', 'O', 'O', 'X', 'X', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ],
                vec![
                    'X', 'X', 'X', 'X', 'X', 'O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X',
                    'X', 'X', 'X', 'X'
                ]
            ]
        );
    }
}