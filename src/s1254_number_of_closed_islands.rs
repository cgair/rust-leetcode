/**
 * [1254] Number of Closed Islands
 *
 * Given a 2D grid consists of 0s (land) and 1s (water). An island is a maximal 4-directionally connected group of 0s 
 * and a closed island is an island totally (all left, top, right, bottom) surrounded by 1s.
 * Return the number of closed islands.
 *
 * 
 * Example 1:
 *
 *
 * Input:
 * 11111110
 * 10000110
 * 10101110
 * 10000101
 * 11111110
 *
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input:
 * 00100
 * 01010
 * 01110
 *
 * 
 * Output: 1
 *
 */

// problem: https://leetcode.cn/problems/number-of-closed-islands/

// submission codes start here
pub struct Solution;

impl Solution {
    // 靠边的陆地不算作「封闭岛屿」
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        for idx in 0..n {
            // 把靠上边的岛屿淹掉
            Solution::dfs(&mut grid, 0, idx as i32, m, n);
            // 把靠下边的岛屿淹掉
            Solution::dfs(&mut grid, m as i32 - 1, idx as i32, m, n);
        }

        for idx in 0..m {
            // 把靠左边的岛屿淹掉
            Solution::dfs(&mut grid, idx as i32, 0, m, n);
            // 把靠右边的岛屿淹掉
            Solution::dfs(&mut grid, idx as i32, n as i32 - 1, m, n);
        }
        // 遍历 grid，剩下的岛屿都是封闭岛屿
        let mut ret = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    ret += 1;
                    Solution::dfs(&mut grid, i as i32, j as i32, m, n);
                }
            }
        }
        
        ret
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32, m: usize, n: usize) {
        if i < 0 || j < 0 || i >= m as i32 || j >= n as i32 { return; }
        if grid[i as usize][j as usize] == 1 { return; }

        grid[i as usize][j as usize] = 1;
        Solution::dfs(grid, i - 1, j, m, n);
        Solution::dfs(grid, i + 1, j, m, n);
        Solution::dfs(grid, i, j - 1, m, n);
        Solution::dfs(grid, i, j + 1, m, n);
    }
}

#[test]
fn test_1254() {
    assert_eq!(
        Solution::closed_island(vec![
            vec![1,1,1,1,1,1,1,0],
            vec![1,0,0,0,0,1,1,0],
            vec![1,0,1,0,1,1,1,0],
            vec![1,0,0,0,0,1,0,1],
            vec![1,1,1,1,1,1,1,0]
            ]),
        2
    );
    assert_eq!(
        Solution::closed_island(vec![vec![0,0,1,0,0], vec![0,1,0,1,0], vec![0,1,1,1,0]]),
        1
    );
    assert_eq!(
        Solution::closed_island(vec![
            vec![1,1,1,1,1,1,1],
            vec![1,0,0,0,0,0,1],
            vec![1,0,1,1,1,0,1],
            vec![1,0,1,0,1,0,1],
            vec![1,0,1,1,1,0,1],
            vec![1,0,0,0,0,0,1],
            vec![1,1,1,1,1,1,1]]
        ),
        2
    );
}