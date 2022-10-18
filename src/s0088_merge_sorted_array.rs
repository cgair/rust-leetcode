/**
 * [88] Merge Sorted Array
 *
 * Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one sorted array.
 *
 * Note:
 *
 *
 * 	The number of elements initialized in nums1 and nums2 are m and n respectively.
 * 	You may assume that nums1 has enough space (size that is greater or equal to m + n) to hold additional elements from nums2.
 *
 *
 * Example:
 *
 *
 * Input:
 * nums1 = [1,2,3,0,0,0], m = 3
 * nums2 = [2,5,6],       n = 3
 *
 * Output: [1,2,2,3,5,6]
 *
 *
 */

// problem: https://leetcode.com/problems/merge-sorted-array/
pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if nums1.is_empty() || nums2.is_empty() {
            return ;
        }
        // inverted sort
        let (mut i, mut j , mut k) = (m - 1, n - 1, m + n - 1);
        while k >= 0 {
            if i >= 0 && (j < 0 || nums1[i as usize] > nums2[j as usize]) { 
                nums1[k as usize] =  nums1[i as usize];
                i -= 1;
            } else { 
                nums1[k as usize] =  nums2[j as usize];
                j -= 1; 
            }

            k -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_88() {
        let mut vec1 = vec![1, 2, 3, 0, 0, 0];
        let mut vec2 = vec![2, 5, 6];
        Solution::merge(&mut vec1, 3, &mut vec2, 3);
        assert_eq!(vec1, vec![1, 2, 2, 3, 5, 6]);

        let mut vec1 = vec![1, 2, 3];
        let mut vec2 = vec![];
        Solution::merge(&mut vec1, 3, &mut vec2, 0);
        assert_eq!(vec1, vec![1, 2, 3]);

        let mut vec1 = vec![2, 0];
        let mut vec2 = vec![1];
        Solution::merge(&mut vec1, 1, &mut vec2, 1);
        assert_eq!(vec1, vec![1, 2]);

        let mut vec1 = vec![0, 0, 0];
        let mut vec2 = vec![1, 2, 3];
        Solution::merge(&mut vec1, 0, &mut vec2, 3);
        assert_eq!(vec1, vec![1, 2, 3]);

        let mut vec1 = vec![1, 2, 4, 5, 6, 0];
        let mut vec2 = vec![3];
        Solution::merge(&mut vec1, 5, &mut vec2, 1);
        assert_eq!(vec1, vec![1, 2, 3, 4, 5, 6]);
    }
}
