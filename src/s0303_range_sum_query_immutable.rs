/**
 * [303] Range Sum Query - Immutable
 *
 * Given an integer array nums, find the sum of the elements between indices i and j (i &le; j), inclusive.
 *
 * Example:<br>
 *
 * Given nums = [-2, 0, 3, -5, 2, -1]
 *
 * sumRange(0, 2) -> 1
 * sumRange(2, 5) -> -1
 * sumRange(0, 5) -> -3
 *
 *
 *
 * Note:<br>
 * <ol>
 * You may assume that the array does not change.
 * There are many calls to sumRange function.
 * </ol>
 *
 */

// problem: https://leetcode.cn/problems/range-sum-query-immutable/

struct NumArray {
    inner: Vec<i32>,
    pre_sum: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut pre_sum = Vec::with_capacity(nums.len() + 1);
        let inner = nums.clone();
        let mut sum = 0;
        pre_sum.push(0);
        for x in nums {
            sum += x;
            pre_sum.push(sum);
        }
        Self { 
            inner,
            pre_sum
        }
    }
    
    // O(N) 效率差 (因为会被频繁调用), N: 数组长度
    // fn sum_range(&self, left: i32, right: i32) -> i32 {
    //     assert!(left <= right, "left <= right");
    //     let mut sum = 0;
    //     for i in left..(right + 1) {
    //         sum += self.inner[i as usize];
    //     }

    //     sum
    // }
    
    // 如何降到 O(1), (不用 for 循环)
    // 核心思路: 用一个 presum 数组记录 nums[0..i - 1] 的和
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        assert!(left <= right, "left <= right");
        self.pre_sum[(right + 1) as usize] - if left > 0 { self.pre_sum[left as usize] } else { 0 }
    }
}

/*
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_303() {
        let mut nums = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(nums.sum_range(0, 2), 1);
        assert_eq!(nums.sum_range(2, 5), -1);
        assert_eq!(nums.sum_range(0, 5), -3);
    }
}