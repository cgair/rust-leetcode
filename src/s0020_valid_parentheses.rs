/**
 * [20] Valid Parentheses
 *
 * Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
 *
 * An input string is valid if:
 *
 * 	Open brackets must be closed by the same type of brackets.
 * 	Open brackets must be closed in the correct order.
 *
 * Note that an empty string is also considered valid.
 *
 * Example 1:
 *
 *
 * Input: "()"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: "()[]{}"
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: "(]"
 * Output: false
 *
 *
 * Example 4:
 *
 *
 * Input: "([)]"
 * Output: false
 *
 *
 * Example 5:
 *
 *
 * Input: "{[]}"
 * Output: true
 *
 *
 */
pub struct Solution;

// problem: https://leetcode.cn/problems/valid-parentheses/

// submission codes start here

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.is_empty() { return true; }
        let mut stack = vec![];
        let s_vec = s.chars().collect::<Vec<_>>();
        for ch in s_vec {
            match ch {
                '(' | '{' | '[' => {
                    stack.push(ch);
                },
                ')' | '}' | ']' => {
                    // 想太多了 :(
                    // let mut tmp = vec![];
                    // while let Some(cch) = stack.pop() {
                    //     if !Solution::pair(cch, ch) {
                    //         tmp.push(cch);
                    //     } else { break; }
                    // }
                    // while let Some(ccch) = tmp.pop() {
                    //     stack.push(ccch);
                    // }
                    if let Some(cch) = stack.pop() {
                        if !Solution::pair(cch, ch) { return false; }
                    } else { return false; }
                },
                _ => { panic!("Invalid string"); }
            }
        }

        true
    }

    #[inline(always)]
    fn pair(open: char, close: char) -> bool {
        (open == '{' && close == '}')
            || (open == '(' && close == ')')
            || (open == '[' && close == ']')
    }
}


#[test]
fn test_20() {
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    assert_eq!(Solution::is_valid("([)]".to_string()), false);
}