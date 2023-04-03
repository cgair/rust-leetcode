/**
 * [150] Evaluate Reverse Polish Notation
 *
 * Evaluate the value of an arithmetic expression in [Reverse Polish Notation](http://en.wikipedia.org/wiki/Reverse_Polish_notation).
 *
 * Valid operators are +, -, *, /. Each operand may be an integer or another expression.
 *
 * Note:
 *
 *
 * 	Division between two integers should truncate toward zero.
 * 	The given RPN expression is always valid. That means the expression would always evaluate to a result and there won't be any divide by zero operation.
 *
 *
 * Example 1:
 *
 *
 * Input: ["2", "1", "+", "3", "*"]
 * Output: 9
 * Explanation: ((2 + 1) * 3) = 9
 *
 *
 * Example 2:
 *
 *
 * Input: ["4", "13", "5", "/", "+"]
 * Output: 6
 * Explanation: (4 + (13 / 5)) = 6
 *
 *
 * Example 3:
 *
 *
 * Input: ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
 * Output: 22
 * Explanation:
 *   ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
 * = ((10 * (6 / (12 * -11))) + 17) + 5
 * = ((10 * (6 / -132)) + 17) + 5
 * = ((10 * 0) + 17) + 5
 * = (0 + 17) + 5
 * = 17 + 5
 * = 22
 *
 *
 */
use std::str::FromStr;
pub struct Solution;

// problem: https://leetcode.cn/problems/evaluate-reverse-polish-notation/

// submission codes start here

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut operands = Vec::new();
        for t in tokens {
            let t = t.as_str();
            match t {
                "+" | "-" | "*" | "/" => {
                    let rhs = operands.pop().unwrap();
                    let lhs = operands.pop().unwrap();
                    match t {
                        "+" => { operands.push(lhs + rhs); },
                        "-" => { operands.push(lhs - rhs); },
                        "*" => { operands.push(lhs * rhs); },
                        "/" => { operands.push(lhs / rhs); },
                        _ => { panic!("Invalid operator"); }
                    }
                },
                _ => {
                    let operand = i32::from_str(t).unwrap();
                    operands.push(operand);
                }
            }
        }

        operands.pop().unwrap()
    }
}

// submission codes end
use crate::vec_string;
#[test]
fn test_150() {
    assert_eq!(
        Solution::eval_rpn(vec_string![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
        ]),
        22
    );
}