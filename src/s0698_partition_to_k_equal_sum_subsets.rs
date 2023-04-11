/**
 * [698] Partition to K Equal Sum Subsets
 *
 * Given an integer array nums and an integer k, 
 * return true if it is possible to divide this array into k non-empty subsets whose sums are all equal.
 * 
 *
 * Example1:
 * 
 * 
 * Input: nums = [4,3,2,3,5,2,1], k = 4
 * Output: true
 *
 * 
 * Explanation: It is possible to divide it into 4 subsets (5), (1, 4), (2,3), (2,3) with equal sums.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [1,2,3,4], k = 3
 * Output: false.
 *
 *
 */

pub struct Solution;

// problem: https://leetcode.cn/problems/partition-to-k-equal-sum-subsets/

impl Solution {
    // 把装有 n 个数字的数组 nums 分成 k 个和相同的集合:
    // 你可以想象将 n 个数字分配到 k 个「桶」里, 最后这 k 个「桶」里的数字之和要相同.
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        if k > nums.len() as i32 { return false; }
        let sum = nums.iter().sum::<i32>();
        if sum % k != 0 { return false; } // k | sum
        let mut nums = nums;
        // <https://stackoverflow.com/a/60916195/19850482>
        use std::cmp::Reverse;
        nums.sort_by_key(|&w| Reverse(w));

        // k 个桶 (集合), 记录每个桶装的数字之和
        let mut bucket = vec![0;k as usize];
        // let mut bucket = Vec::with_capacity(k as usize);
        // 理论上每个桶 (集合) 中数字的和
        let target = sum / k;

        Solution::backtrack(&nums, target, 0, &mut bucket)
    }

    fn backtrack(choices: &[i32], target: i32, start: usize, bucket: &mut Vec<i32>) -> bool {
        if start == choices.len() {
            // 检查所有桶的数字之和是否都是 target
            for &s in bucket.iter() {
                if s != target { return false; }
            }
            return true;
        }

        for b in 0..bucket.len() {
        // 代码 (line 65): 如果我们让尽可能多的情况命中剪枝的那个 if 分支, 就可以减少递归调用的次数, 一定程度上减少时间复杂度.
        // 如何尽可能多的命中这个 if 分支呢? 要知道我们的 index 参数是从 0 开始递增的, 也就是递归地从 0 开始遍历 nums 数组.
        // 如果我们提前对 nums 数组排序, 把大的数字排在前面, 那么大的数字会先被分配到 bucket 中, 对于之后的数字, bucket[i] + nums[index] 会更大, 更容易触发剪枝的 if 条件.
            if bucket[b] + choices[start] > target { continue; }
            // 将 nums[idx] 装入 bucket[b]
            bucket[b] += choices[start];
            if Solution::backtrack(choices, target, start + 1, bucket) {
                return true;
            }
            bucket[b] -= choices[start];
        }
        println!("{bucket:?}");

        // nums[start] 装入哪个桶都不行
        false
    }
}


#[test]
fn test_698() {
    assert_eq!(
        Solution::can_partition_k_subsets(vec![4,3,2,3,5,2,1], 4),
        true
    );

    assert_eq!(
        Solution::can_partition_k_subsets(vec![1,2,3,4], 3),
        false
    );
}