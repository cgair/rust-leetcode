/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.
 *
 * Example 1:
 *
 *
 * Input: "babad"
 * Output: "bab"
 * Note: "aba" is also a valid answer.
 *
 *
 * Example 2:
 *
 *
 * Input: "cbbd"
 * Output: "bb"
 *
 *
 */

// problem: https://leetcode.cn/problems/longest-palindromic-substring/
pub struct Solution;

struct LongestPalindrome(usize, String);


// impl Solution {
//     pub fn longest_palindrome(s: String) -> String {
//         let s_vec = s.chars().collect::<Vec<char>>();
//         let len = s_vec.len();
//         if len == 0 { 
//             return "".to_string() 
//         }
//         let mut ret = LongestPalindrome(0, s_vec[0].to_string());
//         let mut right = len;
//         for left in 0..len - 1 {
//             while left < right {
//                 if Self::_is_palindrome(&s_vec[left..right]) && right - left > ret.0 {
//                     ret.0 = right - left;
//                     ret.1 =  s_vec[left..right].iter().collect::<String>();
//                 }
//                 right -= 1;
//             }
//             right = len;
//         }

//         ret.1
//     }

//     fn _is_palindrome(v: &[char]) -> bool {
//         println!("chars: {:?}", v);
//         let len = v.len();
//         if len == 1 {
//             return false;
//         }
//         let (mut front, mut back) = (0usize, len - 1);
//         while front < back {
//             if v[front] == v[back] {
//                 front += 1;
//                 back -= 1;
//             } else {
//                 return false;
//             }
//         }
//         return true;
//     }
// }

impl Solution {
    // 超出时间限制
    // pub fn longest_palindrome(s: String) -> String {
    //     let len = s.len();
    //     if len == 0 { return  String::from(""); }
    //     let mut max = "";
        
    //     for i in 0.. {
    //         let mut lo = i;
    //         let mut hi = len - 1;
    //         while lo < hi {
    //             if s[lo..=hi] == s[lo..=hi].chars().rev().collect::<String>() {
    //                 max = &s[lo..=hi];
    //                 break;
    //             }
    //             lo += 1;
    //         }
    //         lo = i;
    //         while lo < hi {
    //             if s[lo..=hi] == s[lo..=hi].chars().rev().collect::<String>() && hi - lo + 1 > max.len(){
    //                 max = &s[lo..=hi];
    //                 break;
    //             }
    //             hi -= 1;
    //         }
    //         if max.len() > 0 { break; }
    //     }

    //     max.to_string()
    // }
    
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        assert_eq!(Solution::longest_palindrome("aaaaa".to_owned()), "aaaaa");
        assert_eq!(Solution::longest_palindrome("babab".to_owned()), "babab");
        assert_eq!(Solution::longest_palindrome("babcd".to_owned()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("bb".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("".to_owned()), "");

        // assert_eq!(
        //     true,
        //     Solution::_is_palindrome(&['a', 'b', 'a'])
        // );
        // assert_eq!(
        //     true,
        //     Solution::_is_palindrome(&['a', 'b', 'b', 'a'])
        // );
        // assert_eq!(
        //     false,
        //     Solution::_is_palindrome(&['a', 'b', 'c', 'a'])
        // );
    }
}