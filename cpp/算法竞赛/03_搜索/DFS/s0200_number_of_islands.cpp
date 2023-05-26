// 200. 岛屿数量
// problem: https://leetcode.cn/problems/number-of-islands/
#include <vector>
using namespace std;

class Solution {
public:
    int numIslands(vector<vector<char>>& grid) {
        int ret = 0;
        if (grid.empty()) return ret;

        int m = grid.size(), n = grid[0].size();
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == '1') {
                    dfs(grid, i, j, m, n);
                    ret++;
                }
            }
        }

        return ret;
    }

    void dfs(vector<vector<char>>& grid, int i, int j, int m, int n) {
        if (i < 0 || j < 0 || i >= m || j >= n) return;
        if (grid[i][j] == '0') return;
        grid[i][j] = '0';

        dfs(grid, i - 1, j , m, n);
        dfs(grid, i + 1, j , m, n);
        dfs(grid, i, j - 1 , m, n);
        dfs(grid, i, j + 1 , m, n);
    }
};