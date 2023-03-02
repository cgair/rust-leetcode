/**
 * [80] Remove Duplicates from Sorted Array II
 * 
 * 使得出现次数超过两次的元素只出现两次, 返回删除后数组的新长度.
 *
 * Given a sorted array nums, remove the duplicates [in-place](https://en.wikipedia.org/wiki/In-place_algorithm) such that duplicates appeared at most twice and return the new length.
 *
 * Do not allocate extra space for another array, you must do this by modifying the input array [in-place](https://en.wikipedia.org/wiki/In-place_algorithm) with O(1) extra memory.
 *
 * Example 1:
 *
 *
 * Given nums = [1,1,1,2,2,3],
 *
 * Your function should return length = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
 *
 * It doesn't matter what you leave beyond the returned length.
 *
 * Example 2:
 *
 *
 * Given nums = [0,0,1,1,1,1,2,3,3],
 *
 * Your function should return length = 7, with the first seven elements of nums being modified to 0, 0, 1, 1, 2, 3 and 3 respectively.
 *
 * It doesn't matter what values are set beyond the returned length.
 *
 *
 * Clarification:
 *
 * Confused why the returned value is an integer but your answer is an array?
 *
 * Note that the input array is passed in by reference, which means modification to the input array will be known to the caller as well.
 *
 * Internally you can think of this:
 *
 *
 * // nums is passed in by reference. (i.e., without making a copy)
 * int len = removeDuplicates(nums);
 *
 * // any modification to nums in your function would be known by the caller.
 * // using the length returned by your function, it prints the first len elements.
 * for (int i = 0; i < len; i++) {
 *     print(nums[i]);
 * }
 *
 *
 */
pub struct Solution;

// problem: https://leetcode.cn/problems/remove-duplicates-from-sorted-array-ii/

// submission codes start here

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() { return 0; }
        let (mut slow, mut fast) = (0usize, 0usize);
        let mut memo = 0;   // O(1) extra memory
        while fast < nums.len() {
            if nums[slow] != nums[fast] {
                slow += 1;
                nums[slow] = nums[fast];
            } else if slow < fast && memo < 2 {
                slow += 1;
                nums[slow] = nums[fast];
            }
            memo += 1;
            fast += 1;

            if fast < nums.len() && nums[fast] != nums[fast - 1] { memo = 0; }
        }

        nums.truncate(slow + 1);
        println!("{nums:?}");
        (slow + 1) as i32
    }
}

// submission codes end

#[test]
fn test_80() {
    assert_eq!(
      Solution::remove_duplicates(&mut vec![1,1,1,2,2,3]),  // expected [1,1,2,2,3]
      5
    );
}