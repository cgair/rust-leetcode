// 1020. 飞地的数量
// problem: https://leetcode.cn/problems/number-of-enclaves/
#include <vector>
#include <iostream>
using namespace std;
class Solution {
public:
    int numEnclaves(vector<vector<int>>& grid) {
        int ret = 0;
        if (grid.empty()) return ret;
        int m = grid.size(), n = grid[0].size();
        // flood fill
        for (int i = 0; i < n; ++i) {
            if (grid[0][i] == 1) dfs(grid, 0, i, m, n);
            if (grid[m-1][i] == 1) dfs(grid, m-1, i, m, n);
        }

        for (int i = 0; i < m; ++i) {
            if (grid[i][0] == 1) dfs(grid, i, 0, m, n);
            if (grid[i][n-1] == 1) dfs(grid, i, n-1, m, n);
        }

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 1) {
                    ret ++;
                }
            }
        }
        
        return ret;
    }

    void dfs(vector<vector<int>>& grid, int i, int j, int m, int n) {
        if (i < 0 || i >= m || j < 0 || j >= n) return;
        if (grid[i][j] == 0) return;
        
        grid[i][j] = 0;
        
        dfs(grid, i - 1, j, m, n);
        dfs(grid, i + 1, j, m, n);
        dfs(grid, i, j - 1, m, n);
        dfs(grid, i, j + 1, m, n);
    }
};

int main()
{
    vector<vector<int> > grid = {
        {0,0,0,0},
        {1,0,1,0},
        {0,1,1,0},
        {0,0,0,0}
    };
    Solution s;
    cout << s.numEnclaves(grid) << endl; // 3

    return 0;
}