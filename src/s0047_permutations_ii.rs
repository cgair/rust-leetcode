/**
 * [47] Permutations II
 *
 * Given a collection of numbers that might contain duplicates, return all possible unique permutations.
 *
 * Example:
 *
 *
 * Input: [1,1,2]
 * Output:
 * [
 *   [1,1,2],
 *   [1,2,1],
 *   [2,1,1]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.cn/problems/permutations-ii/

// submission codes start here

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut ret = Vec::new();
        let mut path = Vec::new();
        let mut used = vec![false; nums.len()];
        
        Solution::backtrack(&nums, &mut path, &mut used, &mut ret);
        
        ret
    }

    fn backtrack(choices: &[i32], path: &mut Vec<i32>, used: &mut [bool], ret: &mut Vec<Vec<i32>>) {
        if path.len() == choices.len() {
            ret.push(path.to_owned());
            return;
        }

        for idx in 0..choices.len() {
            if used[idx] { continue; }
            // 新添加的剪枝逻辑, 固定相同的元素在排列中的相对位置
            // 如果前面的相邻相等元素没有用过, 则跳过
            // 标准全排列算法之所以出现重复, 是因为把相同元素形成的排列序列视为不同的序列, 但实际上它们应该是相同的; 而如果固定相同元素形成的序列顺序, 当然就避免了重复.
            if idx > 0 && choices[idx - 1] == choices[idx] && !used[idx - 1] { continue; }
            path.push(choices[idx]);
            used[idx] = true;
            Solution::backtrack(choices, path, used, ret);
            path.pop();
            used[idx] = false;
        }
    }
}


#[test]
fn test_47() {
    assert_eq!(
        Solution::permute_unique(vec![1, 1, 2]),
        vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1],]
    );
    assert_eq!(Solution::permute_unique(vec![1, 1, 1]), vec![vec![1, 1, 1],]);
    assert_eq!(
        Solution::permute_unique(vec![1, 1, 1, 2]),
        vec![
            vec![1, 1, 1, 2],
            vec![1, 1, 2, 1],
            vec![1, 2, 1, 1],
            vec![2, 1, 1, 1],
        ]
    );
    // assert_eq!(
    //     Solution::permute_unique(vec![1, 1, 2, 2, 3, 3]),
    //     vec![
    //         vec![3, 3, 2, 2, 1, 1],
    //         vec![3, 2, 3, 2, 1, 1],
    //         vec![2, 3, 3, 2, 1, 1],
    //         vec![3, 2, 2, 3, 1, 1],
    //         vec![2, 3, 2, 3, 1, 1],
    //         vec![2, 2, 3, 3, 1, 1],
    //         vec![3, 3, 2, 1, 2, 1],
    //         vec![3, 2, 3, 1, 2, 1],
    //         vec![2, 3, 3, 1, 2, 1],
    //         vec![3, 3, 1, 2, 2, 1],
    //         vec![3, 1, 3, 2, 2, 1],
    //         vec![1, 3, 3, 2, 2, 1],
    //         vec![3, 2, 1, 3, 2, 1],
    //         vec![2, 3, 1, 3, 2, 1],
    //         vec![3, 1, 2, 3, 2, 1],
    //         vec![1, 3, 2, 3, 2, 1],
    //         vec![2, 1, 3, 3, 2, 1],
    //         vec![1, 2, 3, 3, 2, 1],
    //         vec![3, 2, 2, 1, 3, 1],
    //         vec![2, 3, 2, 1, 3, 1],
    //         vec![2, 2, 3, 1, 3, 1],
    //         vec![3, 2, 1, 2, 3, 1],
    //         vec![2, 3, 1, 2, 3, 1],
    //         vec![3, 1, 2, 2, 3, 1],
    //         vec![1, 3, 2, 2, 3, 1],
    //         vec![2, 1, 3, 2, 3, 1],
    //         vec![1, 2, 3, 2, 3, 1],
    //         vec![2, 2, 1, 3, 3, 1],
    //         vec![2, 1, 2, 3, 3, 1],
    //         vec![1, 2, 2, 3, 3, 1],
    //         vec![3, 3, 2, 1, 1, 2],
    //         vec![3, 2, 3, 1, 1, 2],
    //         vec![2, 3, 3, 1, 1, 2],
    //         vec![3, 3, 1, 2, 1, 2],
    //         vec![3, 1, 3, 2, 1, 2],
    //         vec![1, 3, 3, 2, 1, 2],
    //         vec![3, 2, 1, 3, 1, 2],
    //         vec![2, 3, 1, 3, 1, 2],
    //         vec![3, 1, 2, 3, 1, 2],
    //         vec![1, 3, 2, 3, 1, 2],
    //         vec![2, 1, 3, 3, 1, 2],
    //         vec![1, 2, 3, 3, 1, 2],
    //         vec![3, 3, 1, 1, 2, 2],
    //         vec![3, 1, 3, 1, 2, 2],
    //         vec![1, 3, 3, 1, 2, 2],
    //         vec![3, 1, 1, 3, 2, 2],
    //         vec![1, 3, 1, 3, 2, 2],
    //         vec![1, 1, 3, 3, 2, 2],
    //         vec![3, 2, 1, 1, 3, 2],
    //         vec![2, 3, 1, 1, 3, 2],
    //         vec![3, 1, 2, 1, 3, 2],
    //         vec![1, 3, 2, 1, 3, 2],
    //         vec![2, 1, 3, 1, 3, 2],
    //         vec![1, 2, 3, 1, 3, 2],
    //         vec![3, 1, 1, 2, 3, 2],
    //         vec![1, 3, 1, 2, 3, 2],
    //         vec![1, 1, 3, 2, 3, 2],
    //         vec![2, 1, 1, 3, 3, 2],
    //         vec![1, 2, 1, 3, 3, 2],
    //         vec![1, 1, 2, 3, 3, 2],
    //         vec![3, 2, 2, 1, 1, 3],
    //         vec![2, 3, 2, 1, 1, 3],
    //         vec![2, 2, 3, 1, 1, 3],
    //         vec![3, 2, 1, 2, 1, 3],
    //         vec![2, 3, 1, 2, 1, 3],
    //         vec![3, 1, 2, 2, 1, 3],
    //         vec![1, 3, 2, 2, 1, 3],
    //         vec![2, 1, 3, 2, 1, 3],
    //         vec![1, 2, 3, 2, 1, 3],
    //         vec![2, 2, 1, 3, 1, 3],
    //         vec![2, 1, 2, 3, 1, 3],
    //         vec![1, 2, 2, 3, 1, 3],
    //         vec![3, 2, 1, 1, 2, 3],
    //         vec![2, 3, 1, 1, 2, 3],
    //         vec![3, 1, 2, 1, 2, 3],
    //         vec![1, 3, 2, 1, 2, 3],
    //         vec![2, 1, 3, 1, 2, 3],
    //         vec![1, 2, 3, 1, 2, 3],
    //         vec![3, 1, 1, 2, 2, 3],
    //         vec![1, 3, 1, 2, 2, 3],
    //         vec![1, 1, 3, 2, 2, 3],
    //         vec![2, 1, 1, 3, 2, 3],
    //         vec![1, 2, 1, 3, 2, 3],
    //         vec![1, 1, 2, 3, 2, 3],
    //         vec![2, 2, 1, 1, 3, 3],
    //         vec![2, 1, 2, 1, 3, 3],
    //         vec![1, 2, 2, 1, 3, 3],
    //         vec![2, 1, 1, 2, 3, 3],
    //         vec![1, 2, 1, 2, 3, 3],
    //         vec![1, 1, 2, 2, 3, 3]
    //     ]
    // );
}