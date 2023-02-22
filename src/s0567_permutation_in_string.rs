/**
 * [567] Permutation in String
 *
 * Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.
 * 
 * In other words, return true if one of s1's permutations is the substring of s2.
 *
 * Example1:
 *
 * 
 * Input: s1 = "ab", s2 = "eidbaooo"
 * Output: true
 * Explanation: s2 contains one permutation of s1 ("ba").
 *
 * 
 * Example1:
 * 
 * 
 * Input: s1 = "ab", s2 = "eidboaoo"
 * Output: false
 */

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut needs = HashMap::new();
        s1.chars().for_each(|c| { needs.entry(c).and_modify(|e| *e += 1).or_insert(1); });

        let (mut left, mut right, mut valid) = (0usize, 0usize, 0);
        let mut window = HashMap::new();

        while right < s2.len() {
            let r = s2.chars().nth(right).unwrap();
            right += 1;
            if needs.contains_key(&r) {
                window.entry(r).and_modify(|e| *e += 1).or_insert(1);
                if window.get(&r) == needs.get(&r) { valid += 1; }
            }

            if right - left >= s1.len() {
                if valid == needs.len() { return true; }
                let l = s2.chars().nth(left).unwrap();
                left += 1;
                if needs.contains_key(&l) {
                    if window.get(&l) == needs.get(&l) { valid -= 1; }
                    window.entry(l).and_modify(|e| *e -= 1);
                }
            }
        }

        false        
    }
}