/**
 * [剑指 Offer 53 - II] 0 ～ n - 1 中缺失的数字
 *
 * 一个长度为 n - 1 的递增排序数组中的所有数字都是唯一的, 
 * 并且每个数字都在范围 0 ～ n - 1 之内. 
 * 在范围 0 ～ n-1 内的 n 个数字中有且只有一个数字不在该数组中.
 * 请找出这个数字.
 * 
 * 
 * 示例1:
 *
 *
 * 输入: [0,1,3]
 * 输出: 2
 * 
 * 
 * 示例 2:
 * 
 * 
 * 输入: [0,1,2,3,4,5,6,7,9]
 * 输出: 8
 * 
 */
// problem: https://leetcode.cn/problems/que-shi-de-shu-zi-lcof/

pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            if i != nums[i] as usize {
                return i as i32;
            }
        }
        nums[nums.len() - 1] + 1
    }

    pub fn missing_number2(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len());        
        while left < right {
            // 二分查找时
            // let mid = (min + max) / 2; 这种写法, 在数据量很大的时候可能会导致 int 类型溢出.
            // 原因: min可能不断增大, 如果到极限状态, 如 min 达到了 max-1 的时候
            // 刚好数组的长度又很大, 那么就可能导致 min + max 的溢出, 所以需要改进.
            // 
            // let mid = (max - min) / 2 + min;
            let mid = left + ((right - left) >> 1);     // 无符号位运算符的优先级较低: 括起来
            if nums[mid] > mid as i32 {
                // mid 和 nums[mid] 不对应, 说明左边有元素缺失
                right = mid;
            } else {
                // mid 和 nums[mid] 对应, 说明元素缺失在右边
                left = mid + 1;
            }
        }

        right as i32
    }
}