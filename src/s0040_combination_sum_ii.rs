/**
 * [40] Combination Sum II
 *
 * Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sums to target.
 *
 * Each number in candidates may only be used once in the combination.
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
 * Input: candidates = [10,1,2,7,6,1,5], target = 8,
 * A solution set is:
 * [
 *   [1, 7],
 *   [1, 2, 5],
 *   [2, 6],
 *   [1, 1, 6]
 * ]
 *
 *
 * Example 2:
 *
 *
 * Input: candidates = [2,5,2,1,2], target = 5,
 * A solution set is:
 * [
 *   [1,2,2],
 *   [5]
 * ]
 *
 *
 */
pub struct Solution;

// problem: https://leetcode.cn/problems/combination-sum-ii/

// submission codes start here

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();

        let mut ret = Vec::new();
        let mut path = Vec::new();

        Solution::backtrack(&candidates, 0, &mut path, &mut ret, target);

        ret        
    }

    fn backtrack(choices: &[i32], start: usize, path: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>, target: i32) {
        // base case
        // 达到目标和, 找到符合条件的组合
        if path.iter().sum::<i32>() == target {
            ret.push(path.to_owned());
            return;
        }
        // base case
        // 超过目标和, 直接结束
        if path.iter().sum::<i32>() > target { return; }

        for idx in start..choices.len() {
            
            if idx > start && choices[idx] == choices[idx - 1] { continue; }
            path.push(choices[idx]);
            Solution::backtrack(choices, idx + 1, path, ret, target);
            path.pop();
        }
    }
}


#[test]
fn test_40() {
    assert_eq!(
        Solution::combination_sum2(vec![1, 1, 1, 1, 1, 1, 1], 7),
        vec![vec![1, 1, 1, 1, 1, 1, 1]]
    );
    assert_eq!(
        Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
        vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6],]
    );
    assert_eq!(
        Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
        vec![vec![1, 2, 2], vec![5],]
    );
}