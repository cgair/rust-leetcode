// 153. 寻找旋转排序数组中的最小值
// problem: https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array/
// 你必须设计一个时间复杂度为 O(log n) 的算法解决此问题。
#include <vector>
using namespace std;

class Solution {
public:
    int findMin(vector<int>& nums) {
        if (nums.empty()) return 0;
        int left = 0, right = nums.size() - 1;
        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (nums[mid] < nums[left]) {
                right = mid;
            } else if (nums[mid] > nums[right]) {  
                left = mid + 1;
            } else {
                return nums[left];
            }
        }

        return nums[left];
    }
};