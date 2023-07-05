// 287. 寻找重复数
// problem: https://leetcode.cn/problems/find-the-duplicate-number/
#include <vector>
using namespace std;

// 你设计的解决方案必须 不修改 数组 nums 且只用常量级 O(1) 的额外空间.

class Solution {
public:
    int findDuplicate(vector<int>& nums) {
        int slow = nums[0], fast = nums[0];
        while (slow != fast) {
            slow = nums[slow];
            fast = nums[nums[fast]];
        }

        int p = 0;
        while (p != fast) {
            p = nums[p];
            fast = nums[fast];
        }

        return p;
    }
};
