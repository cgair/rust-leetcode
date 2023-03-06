/**
 * [239] Sliding Window Maximum
 *
 * Given an array nums, there is a sliding window of size k which is moving from the very left of the array to the very right. 
 * You can only see the k numbers in the window. 
 * Each time the sliding window moves right by one position. Return the max sliding window.
 *
 * 
 * Example:
 *
 *
 * Input: nums = [1,3,-1,-3,5,3,6,7], and k = 3
 * Output: [3,3,5,5,6,7]
 * Explanation:
 *
 * Window position                Max
 * ---------------               -----
 * [1  3  -1] -3  5  3  6  7       3
 *  1 [3  -1  -3] 5  3  6  7       3
 *  1  3 [-1  -3  5] 3  6  7       5
 *  1  3  -1 [-3  5  3] 6  7       5
 *  1  3  -1  -3 [5  3  6] 7       6
 *  1  3  -1  -3  5 [3  6  7]      7
 *
 *
 * Note:
 * You may assume k is always valid, 1 < k < nums.len(); 
 * input array's size for non-empty array.
 *
 * Follow up:
 * Could you solve it in linear time?
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/sliding-window-maximum/

// submission codes start here

impl Solution {
    // time limit exceeded
    // 无法单凭移出窗口的那个元素更新窗口的最值, 除非重新遍历所有元素.
    // 但这样的话时间复杂度就上来了.
    // 
    // pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    //     let mut ret = Vec::new();
        
    //     let mut left = 0usize;
    //     let mut right = (k - 1) as usize;

    //     while right < nums.len() {
    //         let mut p = left;
    //         let mut max = i32::MIN;
    //         while p <= right { 
    //             max = std::cmp::max(max, nums[p]);
    //             p += 1;
    //         }
    //         ret.push(max);
    //         left += 1;
    //         right += 1;
    //     }

    //     ret
    // }
    
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use crate::collections::MonotonicQueue;

        let mut window = MonotonicQueue::new();
        let mut ret = Vec::new();

        for i in 0..nums.len() {
            if i < (k - 1) as usize {
                window.push(nums[i]);
            } else {
                // println!("Before: {:?}", window);
                window.push(nums[i]);
                ret.push(*window.max().unwrap());
                window.pop(nums[i - (k - 1) as usize ]);
                // println!(" After: {:?}", window);
            }
        }

        ret
    }
}


#[test]
fn test_239() {
    assert_eq!(
        Solution::max_sliding_window(vec![9, 10, 9, -7, -4, -8, 2, -6], 5),
        vec![10, 10, 9, 2]
    );
    assert_eq!(
        Solution::max_sliding_window(vec![1, 3, 1, 2, 0, 5], 3),
        vec![3, 3, 2, 5]
    );
    assert_eq!(Solution::max_sliding_window(vec![7, 2, 4], 2), vec![7, 4]);
    assert_eq!(Solution::max_sliding_window(vec![1, -1], 1), vec![1, -1]);
    assert_eq!(
        Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
        vec![3, 3, 5, 5, 6, 7]
    );
}