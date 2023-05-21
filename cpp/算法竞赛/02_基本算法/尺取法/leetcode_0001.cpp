/**
 * 1. 两数之和
 * 给定一个整数数组 nums 和一个整数目标值 target,
 * 请你在该数组中找出 和为目标值 target  的那两个整数, 并返回它们的数组下标.
 * 你可以假设每种输入只会对应一个答案. 但是, 数组中同一个元素在答案里不能重复出现.
 * 你可以按任意顺序返回答案.
 * 
 * 
 * 示例 1:
 * 输入: nums = [2,7,11,15], target = 9
 * 输出: [0,1]
 * 解释: 因为 nums[0] + nums[1] == 9, 返回 [0, 1].
 * 
 * 
 * 示例 2: 
 * 输入: nums = [3,2,4], target = 6
 * 输出: [1,2]
*/
// problem: https://leetcode.cn/problems/two-sum/
#include <iostream>
#include <vector>
#include <unordered_map> // 方法3
#include <chrono>
using namespace std;

class Solution {
public:
    // 方法1. 二重循环包里搜索, O(n^2)
    // 方法2. 二分法: 首先对数组排序「O(nlogn)」, 然而从头到尾扫描每个元素,
    //       在大于 nums[i] 的数中二分查找 target - nums[i]「Olog(n)」, 总复杂度 O(nlogn)
    // 方法3. 哈希 (分配哈希空间) O(n) 空间换时间
    vector<int> twoSum1(vector<int>& nums, int target) {
        vector<int> ret;
        unordered_map<int, int> umap;
        for (int i = 0; i < nums.size(); ++i) {
            if (umap.find(target - nums[i]) == umap.end()) {
                umap[nums[i]] = i;
            } else {
                ret.push_back(i);
                ret.push_back(umap[target - nums[i]]);
                break;
            }
        }

        return ret;
    }

    // 方法4. 尺取法: 首先对数组从大到小排序 (n 个数), 然后反向扫描数组 (i = 0, j = n - 1),
    //       检查 nums[i] + nums[j], 如果大于 target, j --; 如果小于, i++, 直至 nums[i] + nums[j] = target
    // 注意❗️ 这打乱了原数组的顺序, 返回下标的时候要添加额外信息.
    vector<int> twoSum2(vector<int>& nums, int target) {
        vector<int> ret;
        stable_sort(nums.begin(), nums.end());
        int i = 0, j = nums.size() - 1;
        while (i < j) {
            // int sum = nums[i] + nums[j];
            if (nums[i] + nums[j] > target) j --;
            if (nums[i] + nums[j] < target) i ++;
            if (nums[i] + nums[j] == target) {
                ret.push_back(i);
                ret.push_back(j);
                break;
            }
        }
        
        return ret;
    }
};

int main()
{

    return 0;
}