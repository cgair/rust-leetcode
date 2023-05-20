// problem: https://leetcode.cn/problems/max-area-of-island/
#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
    int maxAreaOfIsland(vector<vector<int>>& grid) {
        if(grid.empty()) { return 0; }
        int m = grid.size(), n = grid[0].size();
        int max_area = 0, curr_area = 1;
        for(int i = 0; i < m; ++i) {
            for(int j = 0; j < n; ++j) {
                if(grid[i][j] == 1) {
                    Solution::dfs(grid, i, j, m, n, curr_area);
                    max_area = max(max_area, curr_area);
                    curr_area = 0;
                }
            }
        }

        return max_area;
    }

    void dfs(vector<vector<int>>& grid, int i, int j, int m, int n, int &area) {
        if(i < 0 || i >= m || j < 0 || j >= n) { return; }
        if(grid[i][j] == 1) {
            area += 1;
            grid[i][j] = 0;
        }

        Solution::dfs(grid, i + 1, j, m, n, area);
        Solution::dfs(grid, i - 1, j, m, n, area);
        Solution::dfs(grid, i, j + 1, m, n, area);
        Solution::dfs(grid, i, j - 1, m, n, area);
    }
};

int main() 
{
    return 0;
} 