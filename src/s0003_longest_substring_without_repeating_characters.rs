/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string, find the length of the longest substring without repeating characters.
 *
 * Example:
 *
 * Input: "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 */
use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: &str) -> u32 {
        let s_vec: Vec<char> = s.chars().into_iter().collect();
        let mut lookup = HashSet::new();
        let (mut left, mut max_len, mut right) = (0, 0, 0);
        while right < s_vec.len() {
            let c = s_vec[right];
            right += 1;

            println!("[+] window: [{}, {})", left, right);
            while !lookup.insert(c) {
                // flag = true;
                let d = s_vec[left];
                lookup.remove(&d);
                left += 1;
            }
            max_len = std::cmp::max(max_len, right - left);     // update max len here guarantee all ele can insert into lookup

        }
        max_len as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // chars(&self)
    fn test_string_methods() {
        let word = "goodbye";

        let count = word.chars().count();
        assert_eq!(7, count);

        let mut word = "hello".chars();
        assert_eq!(Some('h'), word.next());
        assert_eq!(Some('e'), word.next());
        assert_eq!(Some('l'), word.next());
        assert_eq!(Some('l'), word.next());
        assert_eq!(Some('o'), word.next());
    }

    #[test]
    // for_each<F>(self, f: F)
    // where
    // F: FnMut(Self::Item), 
    fn test_iterator_trait() {
        let v: Vec<_> =vec![0, 1, 2, 3, 4, 5];
        v.iter().enumerate().for_each(|(i, v)| {
            assert_eq!(0, *v);
        });
    }

    #[test]
    fn test_3() {
        assert_eq!(3, Solution::length_of_longest_substring("abcabcbb"));
        assert_eq!(2, Solution::length_of_longest_substring("ab"));
        assert_eq!(Solution::length_of_longest_substring("bbbb"), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew"),
            3
        );
    }
}
