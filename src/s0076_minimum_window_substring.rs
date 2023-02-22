/**
 * [76] Minimum Window Substring
 *
 * Given a string S and a string T, find the minimum window in S which will contain all the characters in T in complexity O(n).
 *
 * Example:
 *
 *
 * Input: S = "ADOBECODEBANC", T = "ABC"
 * Output: "BANC"
 *
 *
 * Note:
 *
 *
 * 	If there is no such window in S that covers all characters in T, return the empty string "".
 * 	If there is such window, you are guaranteed that there will always be only one unique minimum window in S.
 *
 *
 */

// problem: https://leetcode.cn/problems/minimum-window-substring/

// submission codes start here
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() == 1 && t.len() == 1 && s == t { return s; }

        // 记录最小覆盖子串的起始索引及长度
        // let mut start = 0;
        // let mut len = usize::MAX;
        let mut ret = String::from("");

        let (mut left, mut right, mut valid) = (0usize, 0usize, 0);
        let mut needs = HashMap::new();
        let mut first = true;
        t.chars().for_each(|c| { needs.entry(c).and_modify(|counter| *counter += 1).or_insert(1); } );

        let mut window = HashMap::new();

        while right < s.len() {
            let r = s.chars().nth(right).unwrap();
            right += 1;

            if needs.contains_key(&r) {
                window.entry(r).and_modify(|counter| *counter +=1 ).or_insert(1);
                if window.get(&r) == needs.get(&r) { valid += 1; }
            }

            while valid == needs.len() {
                // if right - left < len {
                //     start = left;
                //     len = right - left;
                // }
                if first { 
                    ret = s[left..right].to_string(); 
                    first = false;
                } else {
                    if (&s[left..right]).len() < ret.len() { ret = s[left..right].to_string(); }
                }

                let l = s.chars().nth(left).unwrap();
                left += 1;

                if needs.contains_key(&l) {
                    if window.get(&l) == needs.get(&l) { valid -= 1; }
                    window.entry(l).and_modify(|counter| *counter -=1 );
                }
            }
            
        }

        ret
        // return if len == usize::MAX { String::from("") } else { s[start..(start + len)].to_string() }
    }
}

#[test]
fn test_76() {
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