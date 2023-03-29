/**
 * [剑指 Offer 48] 最长不含重复字符的子字符串
 *
 * 请从字符串中找出一个最长的不包含重复字符的子字符串, 计算该最长子字符串的长度.
 * 
 * 
 * 示例 1:
 * 输入: "abcabcbb"
 * 输出: 3 
 * 解释: 因为无重复字符的最长子串是 "abc", 所以其长度为 3.
 * 
 * 
 * 示例 2:
 * 输入: "bbbbb"
 * 输出: 1
 * 解释: 因为无重复字符的最长子串是 "b", 所以其长度为 1.
 * 
 * 
 * 示例 3:
 * 输入: "pwwkew"
 * 输出: 3
 * 解释: 因为无重复字符的最长子串是 "wke", 所以其长度为 3.
 * 
 * 请注意, 你的答案必须是子串的长度, "pwke" 是一个子序列, 不是子串.
 *
 */
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() { return 0; }
        let len = s.len();
        let s_vec = s.chars().collect::<Vec<_>>();

        let (mut left, mut right) = (0usize, 0usize);
        let mut max = i32::MIN;
        let mut window = HashMap::new();

        while right < len {
            let rc = s_vec[right];
            right += 1;
            while window.contains_key(&rc) {
                let lc = s_vec[left];
                left += 1;
                window.remove(&lc);
            }
            window.insert(rc, 1);
            max = std::cmp::max(max, (right - left) as i32);
        }

        max
    }
}