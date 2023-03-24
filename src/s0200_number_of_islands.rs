/**
 * [200] Number of Islands
 *
 * Given a 2d grid map of '1's (land) and '0's (water), count the number of islands. 
 * An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. 
 * You may assume all four edges of the grid are all surrounded by water.
 *
 * Example 1:
 *
 *
 * Input:
 * 11110
 * 11010
 * 11000
 * 00000
 *
 * Output: 1
 *
 *
 * Example 2:
 *
 *
 * Input:
 * 11000
 * 11000
 * 00100
 * 00011
 *
 * Output: 3
 *
 */

// problem: https://leetcode.cn/problems/number-of-islands/

// submission codes start here
pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let row = grid.len();
        let col = grid[0].len();
        let mut ret = 0;

        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == '1' {
                    ret += 1; // 每发现一个岛屿, 岛屿数量加一
                    // 然后使用 DFS 将岛屿淹了
                    Solution::dfs(&mut grid, i as i32, j as i32, row, col);
                }
            }
        }

        ret
    }

    fn dfs(grid: &mut Vec<Vec<char>>, i: i32, j: i32, m: usize, n: usize) {
        if i < 0 || j < 0 || i >= m as i32 || j >= n as i32 { return; }
        if grid[i as usize][j as usize] == '0' { return; }
        
        grid[i as usize][j as usize] = '0';
        Solution::dfs(grid, i - 1, j, m, n);
        Solution::dfs(grid, i + 1, j, m, n);
        Solution::dfs(grid, i, j - 1, m, n);
        Solution::dfs(grid, i, j + 1, m, n);
    }
}

#[test]
fn test_200() {
    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0',],
            vec!['1', '1', '0', '1', '0',],
            vec!['1', '1', '0', '0', '0',],
            vec!['0', '0', '0', '0', '0',],
        ]),
        1
    );
    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '0', '1', '0',],
            vec!['1', '1', '0', '1', '0',],
            vec!['1', '1', '0', '0', '0',],
            vec!['0', '0', '0', '1', '1',],
        ]),
        3
    );
}