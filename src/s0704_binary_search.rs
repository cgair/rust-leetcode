/**
 * [792] Binary Search
 *
 * Given a sorted (in ascending order) integer array nums of n elements and a target value, write a function to search target in nums. If target exists, then return its index, otherwise return -1.
 *
 * 
 * Example 1:
 *
 *
 * Input: nums = [-1,0,3,5,9,12], target = 9
 * Output: 4
 * Explanation: 9 exists in nums and its index is 4
 *
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [-1,0,3,5,9,12], target = 2
 * Output: -1
 * Explanation: 2 does not exist in nums so return -1
 *
 *
 *  
 *
 * Note:
 *
 * 
 * 	You may assume that all elements in nums are unique.
 * 	n will be in the range [1, 10000].
 * 	The value of each element in nums will be in the range [-9999, 9999].
 * 
 *
 */
// problem: https://leetcode.cn/problems/binary-search/
use std::cmp::Ordering;
pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        let (mut left, mut right) = (0, (len - 1) as i32);
        while left <= right {
            let mid = left + ((right - left) >> 1);
            // more c
            // if nums[mid] == target {
            //     return mid as i32;
            // } else if nums[mid] > target {
            //     right = mid - 1;
            // } else if nums[mid] < target {
            //     left = mid + 1;
            // }
            // more rust
            match nums[mid as usize].cmp(&target) {
                Ordering::Equal => { return mid as i32; },
                Ordering::Greater => { right = mid - 1; },
                Ordering::Less => { left = mid + 1; }
            }
                
        }
        -1
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_704() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(Solution::search(vec![1], 1), 0);
        assert_eq!(Solution::search(vec![5], -5), -1);
        assert_eq!(Solution::search(vec![5], 6), -1);
        assert_eq!(Solution::search(vec![1, 2], 0), -1);
        assert_eq!(Solution::search(vec![1, 2], 1), 0);
        assert_eq!(Solution::search(vec![1, 2], 2), 1);
        assert_eq!(Solution::search(vec![1, 2], 3), -1);
    }
}