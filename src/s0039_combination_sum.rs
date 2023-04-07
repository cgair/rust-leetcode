/**
 * [39] Combination Sum
 *
 * Given a set of candidate numbers (candidates) (without duplicates) and a target number (target), find all unique combinations in candidates where the candidate numbers sums to target.
 *
 * The same repeated number may be chosen from candidates unlimited number of times.
 *
 * Note:
 *
 *
 * 	All numbers (including target) will be positive integers.
 * 	The solution set must not contain duplicate combinations.
 *
 *
 * Example 1:
 *
 *
 * Input: candidates = [2,3,6,7], target = 7,
 * A solution set is:
 * [
 *   [7],
 *   [2,2,3]
 * ]
 *
 *
 * Example 2:
 *
 *
 * Input: candidates = [2,3,5], target = 8,
 * A solution set is:
 * [
 *   [2,2,2,2],
 *   [2,3,3],
 *   [3,5]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/combination-sum/

// submission codes start here

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if candidates.is_empty() { return vec![]; }
        let mut path = Vec::new();
        let mut ret = Vec::new();
        Solution::backtrack(&candidates, &mut path, 0, &mut ret, target);

        ret
    }

    fn backtrack(choices: &[i32], path: &mut Vec<i32>, start: usize, ret: &mut Vec<Vec<i32>>, target: i32) {
        let sum = path.iter().sum::<i32>();
        if sum > target { return; }
        if sum == target { ret.push(path.to_owned()); }

        for idx in start..choices.len() {
            path.push(choices[idx]);
            Solution::backtrack(choices, path, idx, ret, target);
            path.pop().unwrap();
        }
    }
}


// submission codes end
#[test]
fn test_39() {
    assert_eq!(
        Solution::combination_sum(vec![1], 7),
        vec![vec![1, 1, 1, 1, 1, 1, 1]]
    );
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7],]
    );
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 5], 8),
        vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
    );
}