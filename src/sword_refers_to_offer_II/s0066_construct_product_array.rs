/**
 * [剑指 Offer 66] 构建乘积数组
 *
 * 给定一个数组 A[0,1,…,n-1], 
 * 请构建一个数组 B[0,1,…,n-1], 其中 B[i] 的值是数组 A 中除了下标 i 以外的元素的积, 即 B[i]=A[0]×A[1]×…×A[i-1]×A[i+1]×…×A[n-1]. 
 * 不能使用除法.
 * 
 * 
 * 示例:
 *
 *
 * 输入: [1,2,3,4,5]
 * 输出: [120,60,40,30,24]
 * 
 */
// problem: https://leetcode.cn/problems/gou-jian-cheng-ji-shu-zu-lcof/

pub struct Solution;

impl Solution {
    pub fn construct_arr(a: Vec<i32>) -> Vec<i32> {
        if a.is_empty() { return vec![]; }
        let len = a.len();
        // 从左到右的前缀积 prefix[i] 是 [0, i) 的元素的乘积
        let mut prefix = vec![1;len];
        for i in 1..len {
            prefix[i] = prefix[i - 1] * a[i - 1]
        }
        println!("{prefix:?}");

        // 从右到左的后缀积 suffix[i] 是 (i, len - 1] 的元素的乘积
        let mut suffix = vec![1;len];
        for i in (0..len - 1).rev() {
            suffix[i] = suffix[i + 1] * a[i + 1]
        }
        println!("{suffix:?}");

        let mut ret = vec![0;len];
        ret[0] = suffix[0];
        ret[len - 1] = prefix[len - 1];
        for i in 1..len - 1 {
            ret[i] = prefix[i] * suffix[i];
        }

        ret
    }
}


#[test]
fn test_sword_66() {
    assert_eq!(
        Solution::construct_arr(vec![1, 2, 3, 4]),
        vec![24, 12, 8, 6]
    );
}