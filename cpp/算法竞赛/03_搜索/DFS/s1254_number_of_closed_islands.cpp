// 1254. 统计封闭岛屿的数目
// problem: https://leetcode.cn/problems/number-of-closed-islands/
#include <vector>
using namespace std;

class Solution {
public:
    int closedIsland(vector<vector<int>>& grid) {
        int ret = 0;
        if (grid.empty()) return ret;
        int m = grid.size(), n = grid[0].size();
        for (int i = 0; i < n; ++i) {
            if (grid[0][i] == 0) dfs(grid, 0, i, m, n);
            if (grid[m-1][i] == 0) dfs(grid, m-1, i, m, n);
        }

        for (int i = 0; i < m; ++i) {
            if (grid[i][0] == 0) dfs(grid, i, 0, m, n);
            if (grid[i][n-1] == 0) dfs(grid, i, n-1, m, n);
        }

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 0) {
                    ret ++;
                    dfs(grid, i, j, m, n);
                }
            }
        }

        return ret;
    }

    void dfs(vector<vector<int>>& grid, int i, int j, int m, int n) {
        if (i < 0 || j < 0 || i >= m || j >= n) return;
        if (grid[i][j] == 1) return;

        grid[i][j] = 1;

        dfs(grid, i - 1, j, m, n);
        dfs(grid, i + 1, j, m, n);
        dfs(grid, i, j - 1, m, n);
        dfs(grid, i, j + 1, m, n);
    }
};