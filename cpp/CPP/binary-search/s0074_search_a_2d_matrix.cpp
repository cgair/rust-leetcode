// problem: https://leetcode.cn/problems/search-a-2d-matrix/
#include <iostream>
#include <vector>
// use flatten() provided by <ranges> library, which was introduced in C++20.
using namespace std;

class Solution {
public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        if(matrix.empty() || matrix[0].empty()) { return false; }
        // 已知二维数组的行数 `m` 和列数 `n`, 二维坐标 `(i, j)` 可以映射成一维: `index = i * n + j`, 反之亦可得 `i = index / n, j = index % n`.
        vector<int> flatten;
        for(int i = 0; i < matrix.size(); ++i) {
            for(int j = 0; j < matrix[i].size(); ++j) {
                flatten.push_back(matrix[i][j]);
            }
        }
        int ret = binarySearch(flatten, target);
        return ret >= 0 ? true : false;
    }

    int binarySearch(vector<int>& nums, int target) {
        int left = 0, right = nums.size();
        
        while(left < right) {
           int mid = left + (right - left) / 2;
           if(nums[mid] == target) {
            return mid;
           } else if(nums[mid] < target) {
            left = mid + 1;
           } else if(nums[mid] > target) {
            right = mid;
           }
        }

        return -1;
    }
};

int main()
{
    return 0;
}