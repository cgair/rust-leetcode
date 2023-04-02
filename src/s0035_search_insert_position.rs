/**
 * [35] Search Insert Position
 *
 * Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
 *
 * You may assume no duplicates in the array.
 *
 * Example 1:
 *
 *
 * Input: [1,3,5,6], 5
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: [1,3,5,6], 2
 * Output: 1
 *
 *
 * Example 3:
 *
 *
 * Input: [1,3,5,6], 7
 * Output: 4
 *
 *
 * Example 4:
 *
 *
 * Input: [1,3,5,6], 0
 * Output: 0
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/search-insert-position/

// submission codes start here

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] > target {
                right = mid;
            } else if nums[mid] < target {
                left = mid + 1;
            }
        }

        right as i32
    }
}

#[test]
fn test_35() {
    assert_eq!(
        Solution::search_insert(vec![1, 3, 5, 6], 5),
        2
    );
    assert_eq!(
        Solution::search_insert(vec![1, 3, 5, 6], 2),
        1
    );
    assert_eq!(
        Solution::search_insert(vec![1, 3, 5, 6], 7),
        4
    );
    assert_eq!(
        Solution::search_insert(vec![1, 3, 5, 6], 0),
        0
    );
}