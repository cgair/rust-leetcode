/**
 * [875] Koko-eating-bananas
 * Return the minimum integer k such that she can eat all the bananas within h hours.
 * 
 * Example 1:
 * 
 * Input: piles = [3,6,7,11], h = 8
 * Output: 4
 * 
 * Example 2:
 * 
 * Input: piles = [30,11,23,4,20], h = 5
 * Output: 30
 * 
 * Example 3:
 * 
 * Input: piles = [30,11,23,4,20], h = 6
 * Output: 23
 */

pub struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        assert_ne!(piles.len(), 0);
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        let mut ret = 0;

        while left < right {
            let mid = (right - left) / 2 + left;
            // let time = calculate_time(&piles, mid);     // somehow more c
            let time =  piles.iter().map(|pile| {
                (pile + mid -1) / mid
            }).sum::<i32>();    // this is more rust 
            if time <= h {
                ret = mid;
                right = mid;
            } else if time > h {
                left = mid + 1;
            }
        }

        ret
    }

    /// calculating time at speed `speed`.
    pub fn calculate_time(piles: &[i32], speed: i32) -> i32 {
        let mut time = 0;
        for &pile in piles {
            time += (pile + speed - 1) / speed;
        }

        time
    }
}


/*
* 向上取整trick: (pile + mid - 1) / mid)
* pile 如果是 mid 的整倍数, 结果就是 pile / mid, 如果有余数结果是 pile / mid + 1 ; 所以完全可以表示整数相除结果向上取整
*/

#[test]
fn test_min_eating_speed() {
    let piles = vec![(vec![3,6,7,11], 8), (vec![30,11,23,4,20], 5), (vec![30,11,23,4,20], 6)];

    for (p, h) in piles {
        let k = Solution::min_eating_speed(p, h);
        println!("the minimum integer k = {}", k);  // 4, 30, 23
    }
}