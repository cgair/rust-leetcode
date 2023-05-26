// 695. 岛屿的最大面积
// problem: https://leetcode.cn/problems/max-area-of-island/
#include <vector>
using namespace std;

class Solution {
public:
    int maxAreaOfIsland(vector<vector<int>>& grid) {
        int ret = 0, tmp = 0;
        if (grid.empty()) return ret;
        int m = grid.size(), n = grid[0].size();

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 1) {
                    dfs(grid, i, j, m, n, tmp);
                    ret = max(tmp, ret);
                    tmp = 0;
                }
            }
        }

        return ret;
    }

    void dfs(vector<vector<int>>& grid, int i, int j, int m, int n, int& area) {
        if (i < 0 || j < 0 || i >= m || j >= n) return;
        if (grid[i][j] == 0) return;
        grid[i][j] = 0;
        area ++;

        dfs(grid, i - 1, j, m, n, area);
        dfs(grid, i + 1, j, m, n, area);
        dfs(grid, i, j - 1, m, n, area);
        dfs(grid, i, j + 1, m, n, area);
    }
};