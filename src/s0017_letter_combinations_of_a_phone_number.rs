/**
 * [17] Letter Combinations of a Phone Number
 *
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.
 *
 * A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
 * <https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/11/09/200px-telephone-keypad2svg.png>
 *
 * Example:
 *
 *
 * Input: "23"
 * Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
 *
 *
 * Note:
 *
 * Although the above answer is in lexicographical order, your answer could be in any order you want.
 *
 */
pub struct Solution;

// problem: https://leetcode.cn/problems/letter-combinations-of-a-phone-number/

// submission codes start here

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() { return vec![]; }
        let mut path = Vec::new();
        let mut ret = Vec::new();
        let choices = crate::vec_string!("", "",
        "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz");

        Solution::backtrack(&choices, &digits, &mut path, 0, &mut ret, digits.len());

        ret
    }

    fn backtrack(choices: &[String], digits: &String, path: &mut Vec<String>, start: usize, ret: &mut Vec<String>, target: usize) {
        let concat = path.concat();
        if concat.len() == target { 
            ret.push(concat);
            return;
        }

        for idx in start..digits.len() {
            let index = (digits.chars().nth(idx).unwrap() as i32 - 0x30) as usize;
            for ch in choices[index].chars() {
                path.push(ch.to_string());
                Solution::backtrack(choices, digits, path, idx + 1, ret, target);
                path.pop();
            }
        }
    }
}

#[test]
fn test_17() {
    assert_eq!(
        Solution::letter_combinations("23".to_string()),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
}