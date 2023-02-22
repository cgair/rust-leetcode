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
        let mut needs = HashMap::new();
        p.chars().for_each(|c| { needs.entry(c).and_modify(|e| *e +=1).or_insert(1); } );

        let (mut left, mut right, mut valid) = (0usize, 0usize, 0);
        let mut window = HashMap::new();
        let mut ret = vec![];

        while right < s.len() {
            let r = s.chars().nth(right).unwrap();
            right += 1;
            if needs.contains_key(&r) {
                window.entry(r).and_modify(|e| *e += 1).or_insert(1);
                if window.get(&r) == needs.get(&r) { valid += 1; }
            }
            
            if right - left >= p.len() {
                if valid == needs.len() { ret.push(left as i32); }
                let l = s.chars().nth(left).unwrap();
                left += 1;
                if needs.contains_key(&l) {
                    if window.get(&l) == needs.get(&l) {
                        valid -= 1;
                    }
                    window.entry(l).and_modify(|e| *e -= 1);
                }
            }
        }

        ret
    }
} 

#[test]
fn hashmap_api() {
    let mut map = HashMap::new();
    map.insert('a', 1);
    assert_eq!(map.len(), 1);
    map.entry('a').and_modify(|e| *e += 1 );
    assert_eq!(map.len(), 2);
}