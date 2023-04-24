/**
 * [42] Trapping Rain Water
 *
 * Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it is able to trap after raining.
 *
 * The above elevation map is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. 
 * In this case, 6 units of rain water (blue section) are being trapped.
 *
 * Example:
 *
 *
 * Input: [0,1,0,2,1,0,1,3,2,1,2,1]
 * Output: 6
 *
 */
// problem: https://leetcode.cn/problems/trapping-rain-water/
pub struct Solution;

impl Solution {
    // 由浅入深介绍:
    // 暴力解法 -> 备忘录解法 -> 双指针解法,
    // 在 O(N) 时 间 O(1) 空间内解决这个问题.
    // 
    // 就像处理字符串具体到如何处理每一个字符
    // 这里就具体到仅仅对于位置 i, 能装下多少水呢?
    // 位置 i 能达到的水柱高度和其左/右边的最高柱子有关.
    // water[i] = min(
    //            左边最高的柱子 max(height[0..i]), 
    //            右边最高的柱子 max(height[i..end])
    //            ) - height[i]
    // 
    // 暴力解法 这个解法应该是很直接粗暴的
    // 时间复杂度 O(N^2), 空间 复杂度 O(1)
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ret = 0;

        for idx in 1..height.len() - 1 {
            let lmax = {
                let mut h = i32::MIN;
                for i in 0..=idx {
                    h = std::cmp::max(height[i], h);
                }
                h
            };
            let rmax = {
                let mut h = i32::MIN;
                for i in idx..height.len() {
                    h = std::cmp::max(height[i], h);
                }
                h
            };
            ret += std::cmp::min(lmax, rmax) - height[idx];
        }

        ret
    }

    // 备忘录优化: 暴力解法在每个位置 i 都要计算 r_max 和 l_max
    // 直接把结果都缓存下来, 时间复杂度不就降下来了嘛.
    // 时间复杂度降低为 O(N), 已经是最优了,
    // 但是空间复杂度是 O(N).
    pub fn trap2(height: Vec<i32>) -> i32 {
        let mut ret = 0;  
        let len  = height.len();

        let mut l_max = vec![0;len];
        let mut r_max = vec![0;len];
        l_max[0] = height[0];
        r_max[len - 1] = height[len - 1];

        for i in 1..len {
            l_max[i] = std::cmp::max(height[i], l_max[i - 1]);
        }

        for i in (1..len - 1).rev() {
            r_max[i] = std::cmp::max(height[i], r_max[i + 1]);
        }
        
        for idx in 1..len - 1 {
            ret += std::cmp::min(l_max[idx], r_max[idx]) - height[idx];
        }
        
        ret
    }

    // 这次也不要用备忘录提前计算了
    // 而是用双指针边走边算, 
    // 节省下空间复杂度.
    pub fn trap3(height: Vec<i32>) -> i32 {
        let mut ret = 0;
        let l = height.len();
        let (mut left, mut right) = (0, l - 1);
        let mut l_max = height[0];
        let mut r_max = height[l - 1];

        while left <= right {
            l_max = std::cmp::max(l_max, height[left]);
            r_max = std::cmp::max(r_max, height[right]);
            if l_max < r_max {
                ret += l_max - height[left];
                left += 1;
            } else {
                ret += r_max - height[right];
                right -= 1;
            }
        }

        ret
    }

}


#[test]
fn test_42() {
    assert_eq!(
        Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]),
        6
    );
    assert_eq!(
        Solution::trap3(vec![4,2,0,3,2,5]),
        9
    );
}