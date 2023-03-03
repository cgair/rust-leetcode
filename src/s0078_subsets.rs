/**
 * [78] Subsets
 *
 * Given a set of distinct integers, nums, return all possible subsets (the power set).
 *
 * Note: The solution set must not contain duplicate subsets.
 *
 * Example:
 *
 *
 * Input: nums = [1,2,3]
 * Output:
 * [
 *   [3],
 *   [1],
 *   [2],
 *   [1,2,3],
 *   [1,3],
 *   [2,3],
 *   [1,2],
 *   []
 * ]
 *
 */
pub struct Solution;

// problem: https://leetcode.cn/problems/subsets/

// submission codes start here

impl Solution {
    // 组合和子集是一样的:
    // 大小为 k 的组合就是大小为 k 的子集.
    // 
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut path = Vec::new();

        Solution::backtrack(&nums, 0, &mut path, &mut ret);

        ret
    }

    fn backtrack(choices: &[i32], start: usize, path: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        // 如果把根节点作为第 0 层, 将每个节点和根节点之间树枝上的元素作为该节点的值, 那么第 n 层的所有节点就是大小为 n 的所有子集.
        ret.push(path.to_owned());

        for idx in start..choices.len() {
            path.push(choices[idx]);
            Solution::backtrack(choices, idx + 1, path, ret);
            path.pop();
        }
    }
}

#[test]
fn test_78() {
    assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
    assert_eq!(Solution::subsets(vec![1]), vec![vec![], vec![1]]);
    assert_eq!(
        Solution::subsets(vec![1, 2]),
        vec![vec![], vec![1], vec![1, 2], vec![2]]
    );
}