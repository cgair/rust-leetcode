/**
 * [167] Two Sum II - Input array is sorted
 *
 * Given an array of integers that is already sorted in ascending order, find two numbers such that they add up to a specific target number.
 *
 * The function twoSum should return indices of the two numbers such that they add up to the target, where index1 must be less than index2.
 *
 * Note:
 *
 *
 * 	Your returned answers (both index1 and index2) are not zero-based.
 * 	You may assume that each input would have exactly one solution and you may not use the same element twice.
 *
 *
 * Example:
 *
 *
 * Input: numbers = [2,7,11,15], target = 9
 * Output: [1,2]
 * Explanation: The sum of 2 and 7 is 9. Therefore index1 = 1, index2 = 2.
 *
 */
// problem: https://leetcode.cn/problems/two-sum-ii-input-array-is-sorted/

// submission codes start here
use std::cmp::Ordering;
pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut lo = 0usize;
        let mut hi = numbers.len() - 1;

        while lo < hi {
            // 问题应该聚焦于如何利用数组 ascending order 的性质
            match (numbers[lo] + numbers[hi]).cmp(&target) {
                Ordering::Equal => { return vec![(lo + 1) as i32, (hi + 1) as i32]; },  // 题目要求的索引是从 1 开始的
                Ordering::Less => { 
                    lo += 1;    // 让 sum 大一点
                    continue;
                },
                Ordering::Greater => {
                    hi -= 1;    // 让 sum 小一点
                    continue;
                }
            }
        }

        vec![]
    }
}


#[test]
fn test_167() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
}