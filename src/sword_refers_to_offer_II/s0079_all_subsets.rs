/**
 * [剑指 Offer II 079] 所有子集
 *
 * 给定一个整数数组 nums, 数组中的元素互不相同.
 * 返回该数组所有可能的子集 (幂集).
 * 解集不能包含重复的子集. 
 * 你可以按任意顺序返回解集.
 * 
 * 
 * 示例1:
 *
 *
 * 输入: nums = [1,2,3]
 * 输出: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
 * 
 * 
 * 示例2:
 * 输入: nums = [0]
 * 输出: [[],[0]]
 * 
 *
 */
// problem: https://leetcode.cn/problems/zhong-jian-er-cha-shu-lcof/

pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut path, mut ret) = (vec![], Vec::new());
        Solution::backtrack(&nums, &mut path, 0, &mut ret);
        ret
    }

    fn backtrack(choices: &[i32], path: &mut Vec<i32>, start: usize, ret: &mut Vec<Vec<i32>>) {
        ret.push(path.to_owned());

        for idx in start..choices.len() {
            path.push(choices[idx]);
            Solution::backtrack(choices, path, idx + 1, ret);
            path.pop().unwrap();
        }
    }
}


#[test]
fn test_sword_79() {
    assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
    assert_eq!(Solution::subsets(vec![1]), vec![vec![], vec![1]]);
    assert_eq!(
        Solution::subsets(vec![1, 2]),
        vec![vec![], vec![1], vec![1, 2], vec![2]]
    );
}