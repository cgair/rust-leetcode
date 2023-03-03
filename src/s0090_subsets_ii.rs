/**
 * [90] Subsets II
 *
 * Given a collection of integers that might contain duplicates, nums, return all possible subsets (the power set).
 *
 * Note: The solution set must not contain duplicate subsets.
 *
 * Example:
 *
 *
 * Input: [1,2,2]
 * Output:
 * [
 *   [2],
 *   [1],
 *   [1,2,2],
 *   [2,2],
 *   [1,2],
 *   []
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/subsets-ii/

// submission codes start here

/*
count the repeats of each number,
then in backtracking, each number can be picked up for 0..repeat times
using BTreeMap to preserve order (easy for test)
*/


use std::collections::BTreeMap;
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() { return vec![vec![]]; };

        let mut nums = nums;
        nums.sort();

        let mut ret = Vec::new();
        let mut path = Vec::new();

        Solution::backtrack(&mut nums, 0, &mut path, &mut ret);

        ret
    }

    fn backtrack(choices: &[i32], start: usize, path: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        ret.push(path.to_owned());

        for idx in start..choices.len() {
            // 剪枝逻辑: 值相同的相邻树枝, 只遍历第一条
            if idx > start && choices[idx] == choices[idx - 1] { continue; }
            path.push(choices[idx]);
            Solution::backtrack(choices, idx + 1, path, ret);
            path.pop();
        }
    }
}

#[test]
fn test_90() {
    assert_eq!(
        Solution::subsets_with_dup(vec![1, 2, 2]),
        vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2],
        ]
    );
    assert_eq!(Solution::subsets_with_dup(vec![1]), vec![vec![], vec![1],]);
    assert_eq!(Solution::subsets_with_dup(vec![]), vec![vec![],]);
}