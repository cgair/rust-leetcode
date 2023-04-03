/**
 * [剑指 Offer II 017] 含有所有字符的最短字符串
 *
 * 给定两个字符串 s 和 t, 返回 s 中包含 t 的所有字符的最短子字符串.
 * 如果 s 中不存在符合条件的子字符串, 则返回空字符串 "".
 * 如果 s 中存在多个符合条件的子字符串, 返回任意一个.
 * 
 * 
 * 注意： 对于 t 中重复字符, 我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量.
 * 
 * 
 * 示例1:
 *
 *
 * 输入: s = "ADOBECODEBANC", t = "ABC"
 * 输出: "BANC"
 * 解释: 最短子字符串 "BANC" 包含了字符串 t 的所有字符 'A'、'B'、'C'
 * 
 * 
 * 示例2:
 * 输入: s = "a", t = "a"
 * 输出: "a"
 *
 * 
 * 示例3:
 * 输入: s = "a", t = "aa"
 * 输出: ""
 * 解释: t 中两个字符 'a' 均应包含在 s 的子串中, 因此没有符合条件的子字符串, 返回空字符串.
 */
// problem: https://leetcode.cn/problems/M1oyTv/
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() == 1 && t.len() == 1 && s == t { return s; }

        let mut needs = HashMap::new();
        t.chars().for_each(|c| { needs.entry(c).and_modify(|counter| *counter += 1).or_insert(1); });

        let mut window = HashMap::new();
        let sv = s.chars().collect::<Vec<_>>();
        let (mut left, mut right, mut satisfy) = (0usize, 0usize, 0);
        let mut ret = String::new();
        let mut first = true;
        
        while right < s.len() {
            let rc = sv[right];
            right += 1;
            if needs.contains_key(&rc) {
                window.entry(rc).and_modify(|counter| *counter += 1).or_insert(1);
                if needs[&rc] == window[&rc] { satisfy += 1; }
            }

            while satisfy == needs.len() {
                if first {
                    ret = sv[left..right].iter().collect();
                    first = false;
                } else {
                    if sv[left..right].len() < ret.len() {
                        ret = sv[left..right].iter().collect();
                    }
                }
                let lc = sv[left];
                left += 1;
                if needs.contains_key(&lc) {
                    if needs[&lc] == window[&lc] { satisfy -= 1; }
                    window.entry(lc).and_modify(|counter| *counter -= 1);
                }
            }
        }
        
        ret
    }
}

#[test]
fn test_sword_17() {
    assert_eq!(
        Solution::min_window(String::from("a"), String::from("a")),
        "a".to_string()
    );

    let s = String::from("ADOBECODEBANC");
    let t = String::from("ABC");
    assert_eq!(
        Solution::min_window(s, t),
        "BANC".to_string()
    );
}
