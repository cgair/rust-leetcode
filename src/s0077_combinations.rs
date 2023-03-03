/**
 * [77] Combinations
 *
 * Given two integers n and k, return all possible combinations of k numbers out of 1 ... n.
 *
 * Example:
 *
 *
 * Input: n = 4, k = 2
 * Output:
 * [
 *   [2,4],
 *   [3,4],
 *   [2,3],
 *   [1,2],
 *   [1,3],
 *   [1,4],
 * ]
 *
 *
 */
pub struct Solution;

// problem: https://leetcode.cn/problems/combinations/

// submission codes start here

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let choices = (1..=n).collect::<Vec<_>>();
        let mut path = Vec::new();

        Solution::backtrack(&choices, 0, &mut path, &mut ret, k);

        ret
    }

    // 我们通过保证元素之间的相对顺序不变来防止出现重复的子集.
    fn backtrack(
        choices: &[i32], 
        start: usize, /* 通过 start 参数控制树枝的遍历, 避免产生重复的子集 */
        path: &mut Vec<i32>, 
        ret: &mut Vec<Vec<i32>>, 
        k: i32
    ) {
        if path.len() == k as usize {
            ret.push(path.to_owned()); 
            return;
        }

        for idx in start..choices.len() {
            path.push(choices[idx]);
            Solution::backtrack(choices, idx + 1, path, ret, k);
            path.pop();
        }
    }
}


#[test]
    fn test_77() {
        assert_eq!(
            Solution::combine(3, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
            ]
        );
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        );
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::combine(0, 1), empty);
        assert_eq!(Solution::combine(2, 1), vec![vec![1], vec![2]]);
    }