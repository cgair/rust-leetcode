/**
 * [287] Find the Duplicate Number
 *
 * Given an array nums containing n + 1 integers where each integer is between 1 and n (inclusive), 
 * prove that at least one duplicate number must exist. 
 * Assume that there is only one duplicate number, find the duplicate one.
 *
 * Example 1:
 *
 *
 * Input: [1,3,4,2,2]
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: [3,1,3,4,2]
 * Output: 3
 *
 * Note:
 *
 * 	You must not modify the array (assume the array is read only).
 * 	You must use only constant, O(1) extra space.
 * 	Your runtime complexity should be less than O(n^2).
 * 	There is only one duplicate number in the array, but it could be repeated more than once.
 *
 */
pub struct Solution;

// problem: https://leetcode.cn/problems/find-the-duplicate-number/

// Floyd's Tortoise and Hare: 一个循环检测算法, 其中一个指针的速度是另一个指针的两倍, 
// 一旦它们相遇, 我们可以追溯到循环开始的起点

// In our case, the values of the array is like a pointer, pointing to indices of the array
// which is like nodes
// +-----+-----+-----+-----+-----+
// | 0x1 | 0x3 | 0x4 | 0x2 | 0x2 |  --> value
// +-----+-----+-----+-----+-----+
// |  0  |  1  |  2  |  3  |  4  |  --> index
// +-----+-----+-----+-----+-----+
// 因为每个数都是从 1 到 n, 每个值 (value) 必须指向一个有效的索引 (index),
// 而因为有一个重复的数字, there will be a cycle.
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = nums[0] as usize;
        let mut fast = nums[nums[0] as usize] as usize;

        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
        }

        fast = 0usize;
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
        }

        slow as i32
    }
}


#[test]
fn test_287() {
    assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
    assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    assert_eq!(Solution::find_duplicate(vec![1, 2, 3, 4, 5, 5]), 5);
    assert_eq!(Solution::find_duplicate(vec![5, 1, 2, 3, 4, 5]), 5);
}