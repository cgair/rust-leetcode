/**
 * [283] Move Zeroes
 *
 * Given an array nums, write a function to move all 0's to the end of it 
 * while maintaining the relative order of the non-zero elements.
 *
 * Example:
 *
 *
 * Input: [0,1,0,3,12]
 * Output: [1,3,12,0,0]
 *
 * Note:
 *
 * 	You must do this in-place without making a copy of the array.
 * 	Minimize the total number of operations.
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/move-zeroes/

// submission codes start here

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // Forgot maintaining the relative order of the non-zero elements.
        // 
        // let (mut lo, mut hi) = (0, nums.len() - 1);
        // while nums[hi] == 0 { hi -= 1; }
        
        // while lo <= hi {
        //     if nums[lo] == 0 {
        //         nums.swap(lo, hi);
        //         hi -= 1;
        //     }
        //     lo += 1;
        // }

        /* 我竟然没有想到 */
        // let mut last_none_zero = 0usize;
        // for i in 0..nums.len() {
        //     if nums[i] != 0 {
        //         nums.swap(last_none_zero, i);
        //         last_none_zero += 1;
        //     }
        // }

        /* Method 2: use [27] Remove Element */
        let last_none_zero = crate::s0027_remove_element::Solution::remove_element(nums, 0);
        for i in last_none_zero as usize..nums.len() {
            nums[i] = 0;
        }
    }
}


#[test]
fn test_283() {
    let mut vec = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut vec);
    assert_eq!(vec, vec![1, 3, 12, 0, 0]);
}