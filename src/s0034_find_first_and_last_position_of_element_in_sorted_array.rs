/**
 * [34] Find First and Last Position of Element in Sorted Array
 *
 * Given an array of integers nums sorted in ascending order, find the starting and ending position of a given target value.
 *
 * Your algorithm's runtime complexity must be in the order of O(log n).
 *
 * If the target is not found in the array, return [-1, -1].
 *
 * Example 1:
 *
 *
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 *
 * Example 2:
 *
 *
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 *
 */

// problem: https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/

pub struct Solution;

// submission codes start here

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ret = vec![-1, -1];
        ret[0] = Solution::left_bound(&nums, target);
        ret[1] = Solution::right_bound(&nums, target);

        ret
    }

    fn left_bound(nums: &[i32], target: i32) -> i32 {
        // [left, right)
        let (mut left, mut right) = (0, nums.len());

        while left < right {
            let mut mid = left + (right - left) / 2;
            if nums[mid] == target {
                // 要收紧右侧边界以锁定左侧边界
                right = mid;
            } else if nums[mid] > target {
                right = mid;
            } else if nums[mid] < target {
                left = mid + 1;
            }
        }
        
        if left == nums.len() { return -1; }
        return if nums[left] == target { left as i32 } else { -1 };
    } 

    fn right_bound(nums: &[i32], target: i32) -> i32 {
        // [left, right)
        let (mut left, mut right) = (0, nums.len());

        while left < right {
            let mut mid = left + (right - left) / 2;
            if nums[mid] == target {
                // 收紧左侧边界以锁定右侧边界
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid;
            } else if nums[mid] < target {
                left = mid + 1;
            }
        }

        if left as i32 - 1 < 0 { return -1; }
        if nums[left - 1] == target { return left as i32 - 1; }
        -1
    }

}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, BufRead};
    use std::str::FromStr;

    #[test]
    fn test_34() {
        assert_eq!(
            Solution::search_range(vec![5,7,7,8,8,10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![1], 1),
            vec![0, 0]
        );
        assert_eq!(
            Solution::search_range(vec![1], 0),
            vec![-1, -1]
        );
        assert_eq!(
            Solution::search_range(vec![2, 2], 2),
            vec![0, 1]
        );
        assert_eq!(
            Solution::search_range(vec![2, 2], 1),
            vec![-1, -1]
        );
    }
    
    #[test]
    fn stdin() {
        // <https://stackoverflow.com/questions/30186037/how-can-i-read-a-single-line-from-stdin1>
        let stdin = io::stdin();
        // for line in stdin.lock().lines() {
        //     println!("{}", line.unwrap());
        // }
        // Idiom Read list of integers from stdin
        let mut buf = String::new();
        let size = io::stdin().read_line(&mut buf).unwrap();
        println!("{size}, {}", buf.len());
        buf.remove(buf.len() - 1);
        

        // let nums = buf
        //     .lines()
        //     .map(i32::from_str)
        //     .collect::<Result<Vec<_>, _>>()
        //     .unwrap();

        let nums = buf.split(' ')
            .map(i32::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        for n in nums {
            println!("{n:}");
        }
    }
}