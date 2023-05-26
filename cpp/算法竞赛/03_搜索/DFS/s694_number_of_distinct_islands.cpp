// 694. 不同的岛屿数量
// 输入一个二维矩阵, 0 表示海水, 1 表示陆地, 这次让你计算 不同的 (distinct) 岛屿数量.
// https://labuladong.github.io/algo/di-san-zha-24031/bao-li-sou-96f79/yi-wen-mia-4f482/#不同的岛屿数量
#include <vector>
#include <unordered_set>
#include <string>
using namespace std;

// 关注的是「树枝」(岛屿的遍历顺序), 而不是「节点」(岛屿的每个格子).
// 而遍历顺序是写死在你的递归函数 (dfs) 里面的
class Solution {
public:
    int numDistinctIslands(vector<vector<int>>& grid) {
        if (grid.empty()) return 0;
        unordered_set<string> uset;
        int m = grid.size(), n = grid[0].size();

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 1) {
                    string cur_island;
                    dfs(grid, i, j, m, n, cur_island, "Whatever");
                    uset.emplace(cur_island);
                }
            }
        }

        return uset.size();
    }

    void dfs(vector<vector<int>>& grid, int i, int j, int m, int n, string& path, string dir) {
        if (i < 0 || j < 0 || i >= m || j >= n) return;
        if (grid[i][i] == 0) return;

        grid[i][j] = 0;
        path += dir;

        // WSAD --> UP DOWN LEFT RIGHT
        // -W -S -A -D --> revoke
        dfs(grid, i - 1, j, m, n, path, "W");
        dfs(grid, i + 1, j, m, n, path, "S");
        dfs(grid, i, j - 1, m, n, path, "A");
        dfs(grid, i, j + 1, m, n, path, "D");

        path += "-";
        path += dir;
    }
};