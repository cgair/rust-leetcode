/**
 * [438] Find All Anagrams in a String
 *
 * Given two strings s and p, return an array of all the start indices of p's anagrams in s. You may return the answer in any order.
 * 
 * An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 * 
 * 
 * Example:
 *
 *
 * Input:   
 * s = "cbaebabacd"
 * p = "abc"
 *
 * Output: [0,6]
 * 
 * 
 * Input:   
 * s = "abab"
 * p = "ab"
 *
 * Output: [0,1,2]
 */

// problem: https://leetcode.cn/problems/remove-duplicates-from-sorted-array/
use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s_vec = s.chars().collect::<Vec<_>>();
        let p_vec = p.chars().collect::<Vec<_>>();
        let mut map = HashMap::new();
        let mut window = HashMap::new();
        p_vec.iter().map(|c| {map.insert(c, 1)} ).collect::<Vec<_>>();

        let mut result = Vec::new();
        let (mut left, mut right) = (0usize, 0usize);

        while right < s_vec.len() {
            let c = s_vec[right];
            right += 1;
            if !map.get(&c).is_none() {
                window.insert(c, 1);
            } else {
                left += 1;
                continue;
            }
            
            while right - left >= p_vec.len() {
                let d = s_vec[left];
                left += 1;
            }
        }

        result
    }
}