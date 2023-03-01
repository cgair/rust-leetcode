/**
 * [46] Permutations
 *
 * Given a collection of distinct integers, return all possible permutations.
 *
 * Example:
 *
 *
 * Input: [1,2,3]
 * Output:
 * [
 *   [1,2,3],
 *   [1,3,2],
 *   [2,1,3],
 *   [2,3,1],
 *   [3,1,2],
 *   [3,2,1]
 * ]
 *
 *
 */
// problem: https://leetcode.cn/problems/permutations/

// submission codes start here
pub struct Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let mut path = Vec::new();

        /* 路径中的元素标记为 true, 避免重复使用 */
        let mut used = vec![false; nums.len()];

        Solution::backtrack(&nums, &mut path, &mut ret, &mut used);

        ret
    }

    fn backtrack(chioces: &[i32], path: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>, used: &mut [bool]) {
        if path.len() == chioces.len() {
            ret.push(path.to_owned());
            return;
        }

        /*
        我理解 "将该选择从选择列表移除" 和 "将该选择再加入选择列表" 这两句话太单纯了
        用标记数组来说明第 n 个元素是否使用过.
         */

        for idx in 0..chioces.len() {
            let choice = chioces[idx];
            if !used[idx] {
                path.push(choice);
                used[idx] = true;
            } else {
                continue;
            }
            Solution::backtrack(chioces, path, ret, used);
            path.pop();
            used[idx] = false;
        }
    }
}
