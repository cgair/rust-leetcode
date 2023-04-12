/**
 * [416] Partition Equal Subset Sum
 *
 * Given an integer array nums, 
 * return true if you can partition the array into two subsets 
 * such that the sum of the elements in both subsets is equal or false otherwise.
 * 
 *
 * Example1:
 * 
 * 
 * Input: nums = [1,5,11,5]
 * Output: true
 *
 * 
 * Explanation: The array can be partitioned as [1, 5, 5] and [11].
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [1,2,3,5]
 * Output: false
 * 
 * 
 * Explanation: The array cannot be partitioned into equal sum subsets.
 *
 *
 */

pub struct Solution;

// problem: https://leetcode.cn/problems/partition-equal-subset-sum/

// prerequisite: [698] Partition to K Equal Sum Subsets
// 为什么划分两个相等的子集可以转化成背包问题?
// 
// 0-1 背包问题场景中: 
//     有一个重量 W 的背包和 N 件物品, 第 i 件物品的重量是 weight[i], 价值是value[i];
//     每件物品只能用一次 (放入一次), 每个物品有两个选择:「装进背包」和「不装进背包」.
// 原集合 S 划分成两个相等子集 S_1, S_2 的场景下:
//     先对集合求和, 得出 sum, 把问题转化为背包问题.
//      给一个可装载重量为 sum / 2 的背包和 N 个物品, 每个物品的重量为 nums[i].
//      现在让你装物品, 是否存在一种装法, 能够恰好将背包装满?
//     S 中的每个元素也有两个选择:「装进 S_1」和「不装进 S_1（装进 S_2）」.
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        if nums.len() < 2 { return false; }
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 { return false; }
        let target = sum / 2;

        let len = nums.len();
        // 1. 确定dp数组以及下标的含义: 
        // dp[i][j] = x 表示, 对于前 i 个物品 (i 从 1 开始计数), 当前背包的容量为 j 时, 
        // 若 x 为 true, 则说明可以恰好将背包装满, 若 x 为 false, 则说明不能恰好将背包装满.
        // 当没有物品可选择的时候, 肯定没办法装满背包.
        let mut dp_table = vec![vec![false; target as usize + 1]; len + 1];
        // 背包没有空间的时候就相当于装满了.
        for i in 0..=len { dp_table[i][0] = true; }

        for i in 1..=len {
            for j in 1..=target as usize {
                if j as i32 - nums[i - 1] < 0 {
                    dp_table[i][j] = dp_table[i - 1][j];
                } else {
                    dp_table[i][j] = dp_table[i - 1][j] || dp_table[i - 1][(j as i32 - nums[i - 1]) as usize];
                }
            }
        }
        // println!("{dp_table:?}");

        dp_table[len][target as usize]
    }

    // 回溯暴力搜索
    pub fn can_partition1(nums: Vec<i32>) -> bool {
        if nums.len() < 2 { return false; }
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 { return false; }

        let target = sum / 2;
        let mut bucket = vec![0;2];

        Solution::backtrack(&nums, target, 0, &mut bucket)
    }

    fn backtrack(choices: &[i32], target: i32, index: usize, bucket: &mut Vec<i32>) -> bool {
        if index == choices.len() {
            for &s in bucket.iter() {
                if s != target { return false; }
            }
            return true;
        }

        for idx in 0..bucket.len() {
            if bucket[idx] + choices[index] > target { continue; }
            bucket[idx] += choices[index];
            if Solution::backtrack(choices, target, index + 1, bucket) { return true; }
            bucket[idx] -= choices[index];
        }

        false
    }
}


#[test]
fn test_416() {
    let big_nums = vec![100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,100,99,97];

    use std::time::Instant;
    let start = Instant::now();
    assert_eq!(
        Solution::can_partition1(vec![1,5,11,5]),
        true
    );
    println!("Backtrack: {:?}", start.elapsed());

    // time exceed
    // assert_eq!(
    //     Solution::can_partition1(big_nums),
    //     true
    // );

    let start = Instant::now();
    assert_eq!(
        Solution::can_partition(vec![1,5,11,5]),
        true
    );
    println!("Dynamic Programing: {:?}", start.elapsed());

    assert_eq!(
        Solution::can_partition(big_nums),
        false
    );
}