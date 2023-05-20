// problem: https://leetcode.cn/problems/rotting-oranges/
#include <vector>
#include <queue>
#include <utility>

class Solution {
public:
    int orangesRotting(std::vector<std::vector<int>>& grid) {
        if (grid.empty()) return -1;
        int m = grid.size(), n = grid[0].size();
        std::queue<std::pair<int, int>> q;
        int count = 0, minutes = 0;

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == 2) {
                    q.push(std::make_pair(i, j));
                }
                if (grid[i][j] == 1) {
                    count ++;
                }
            }
        }

        while (!q.empty()) {
            int sz = q.size();
            bool rotten = false;
            for (int i = 0; i < sz; ++i) {
                auto p = q.front();
                q.pop();
                int x = p.first, y = p.second;
                if (x - 1 > 0 && grid[x - 1][y] == 1) {
                    grid[x - 1][y] = 2;
                    q.push({x-1, y});
                    rotten = true;
                }
                if (x + 1 < m && grid[x + 1][y] == 1) {
                    grid[x + 1][y] = 2;
                    q.push({x+1, y});
                    count--;
                    rotten = true;
                }
                if (y - 1 > 0 && grid[x][y - 1] == 1) {
                    grid[x][y - 1] = 2;
                    q.push({x, y - 1});
                    count --;
                    rotten = true;
                }
                if (y + 1 < n && grid[x][y + 1] == 1) {
                    grid[x][y + 1] = 2;
                    q.push({x, y+1});
                    count --;
                    rotten = true;
                }
            }
            if (rotten) minutes ++;
        }

        return count == 0 ? minutes : -1;
    }
};