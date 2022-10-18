
// problem: https://leetcode.cn/problems/next-greater-element-i/
pub struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::with_capacity(nums1.len());
        /* Violence
        for n1 in nums1 {
            let indexes: Vec<_> = nums2.iter()
                .enumerate()
                .map(|(index, &value)| {
                    if value == n1 {
                        index as isize
                    } else {
                        -1 as isize
                    }
                }).collect();
                println!("indexes: {:?}", indexes);
                let start = (indexes.iter().filter(|x| !x.is_negative()).next().unwrap() + 1) as usize;
                let mut max = -1;
                for i in start..nums2.len() {
                    if nums2[i] > n1 {
                        max = nums2[i];
                        break;
                    }
                }
            
            ret.push(max);
        }
        */
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let greater = Solution::_next_greater_element(&nums2);
        for i in 0..nums2.len() {
            map.insert(nums2[i], greater[i]);
        }
        println!("greater = {:?}", greater);
        for n1 in nums1 {
            ret.push(*map.get(&n1).unwrap());
        }

        ret

    }

    fn _next_greater_element(nums: &[i32]) -> Vec<i32> {
        let len = nums.len();
        let mut ret = vec![0;len];
        let mut stack = Vec::new();
        for i in (0..nums.len()).rev() {
            if !stack.is_empty() && stack.last() <= Some(&nums[i]) {
                stack.pop();
            }
            ret[i] = if stack.is_empty() { -1 } else { stack.last().unwrap().clone()};
            stack.push(nums[i]);
        }

        ret
    }
}

#[test]
fn test_496() {
    assert_eq!(
        vec![-1, 3, -1],
        Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
    );
    assert_eq!(
        vec![3, -1],
        Solution::next_greater_element(vec![2, 4], vec![1,2, 3, 4])
    );
}