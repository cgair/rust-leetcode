/**
 * [剑指 Offer 21] 调整数组顺序使奇数位于偶数前面
 *
 * 输入一个整数数组, 实现一个函数来调整该数组中数字的顺序, 使得所有奇数在数组的前半部分, 所有偶数在数组的后半部分.
 * 
 * 
 * 示例1:
 * 
 * 
 * 输入: nums = [1,2,3,4]
 * 输出: [1,3,2,4] 
 * 注: [3,1,2,4] 也是正确的答案之一.
 *
 */
// problem: https://leetcode.cn/problems/diao-zheng-shu-zu-shun-xu-shi-qi-shu-wei-yu-ou-shu-qian-mian-lcof/
pub struct Solution;

fn is_odd(num: i32) -> bool {
    num & 1 == 1
}

impl Solution {
    pub fn exchange(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() { return vec![]; }
        let mut nums = nums;
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo < hi {
            let n1 = nums[lo] % 2;
            let n2 = nums[hi] % 2;
            match (n1, n2) {
                (0, 0) => {
                    hi -= 1;
                },
                (1, 1) => {
                    lo += 1;
                }, 
                (0, 1) => {
                    nums.swap(lo, hi);
                    hi -= 1;
                    lo += 1;
                },
                (1, 0) => {
                    hi -= 1;
                    lo += 1;
                }
                _ => ()
            }
        }
        
        nums
    }
}
