// 1905. 统计子岛屿
// problem: https://leetcode.cn/problems/count-sub-islands/
#include <vector>
using namespace std;

// 1. 借助 Union Find 并查集算法 
// 
// 2. 如果岛屿 B 中存在一片陆地, 在岛屿 A 的对应位置是海水, 那么岛屿 B 就不是岛屿 A 的子岛.
class Solution {
public:
    int countSubIslands(vector<vector<int>>& grid1, vector<vector<int>>& grid2) {
        int ret = 0;
        bool flag = true;
        int m = grid2.size(), n = grid2[0].size();
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid2[i][j] == 1) {
                    dfs(grid1, grid2, i, j, m, n, flag);
                    if (flag) ret++;
                    flag = true;
                }
            }
        }
        // 或者先把一定不是子岛的淹掉
        // 然后问题就变成了计算岛屿数量

        return ret;
    }

    void dfs(vector<vector<int>>& grid1, vector<vector<int>>& grid2, int i, int j, int m, int n, bool& is_sub) {
        if (i < 0 || j < 0 || i >= m || j >= n) return;
        if (grid2[i][j] == 0) return;
        if (grid1[i][j] == 0) {
            is_sub = false;
            return;
        }

        grid2[i][j] = 0;
        dfs(grid1, grid2, i - 1, j, m, n, is_sub);
        dfs(grid1, grid2, i + 1, j, m, n, is_sub);
        dfs(grid1, grid2, i, j - 1, m, n, is_sub);
        dfs(grid1, grid2, i, j + 1, m, n, is_sub);
    }
};