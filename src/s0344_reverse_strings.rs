/**
 * [344] Reverse String
 *
 * Write a function that reverses a string. The input string is given as an array of characters s.
 *
 *
 * Note:
 *
 *
 * 	You must do this by modifying the input array in-place with O(1) extra memory.
 *
 *
 * Example:
 *
 *
 * Input: s = ["h","e","l","l","o"]
 * Output: ["o","l","l","e","h"]
 * 
 * Input: s = ["H","a","n","n","a","h"]
 * Output: ["h","a","n","n","a","H"]
 *
 */
// problem: https://leetcode.cn/problems/reverse-string/

// submission codes start here
pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut lo = 0;
        let mut hi = s.len() - 1;

        while lo < hi {
            // s.swap(lo, hi);
            let tmp = s[lo];
            s[lo] = s[hi];
            s[hi] = tmp;
            
            lo += 1;
            hi -= 1;
        }
    }
}

#[test]
fn test_344() {
    let mut s = vec!['h','e','l','l','o'];
    Solution::reverse_string(&mut s);
    assert_eq!(
        s,
        vec!['o','l','l','e','h']
    );
}