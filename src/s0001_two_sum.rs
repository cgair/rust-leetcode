/**
 * [1] Two Sum
 *
 * Given an array of integers, return indices of the two numbers such that they
 * add up to a specific target.
 *
 * You may assume that each input would have exactly one solution, and you may
 * not use the same element twice.
 *
 * Example:
 *
 *
 * Given nums = [2, 7, 11, 15], target = 9,
 *
 * Because nums[0] + nums[1] = 2 + 7 = 9,
 * return [0, 1].
 *
 */
use std::collections::HashMap;
struct Solution {}

impl Solution {
    pub fn two_sum(array: Vec<i32>, target: i32) -> Vec<i32>{
        let mut map: HashMap<i32, i32> = HashMap::new();
        let size = array.len();
        // for i in 0..size {
        //     map.insert(array[i], i as i32);
        // }
        // for i in 0..size {
        //     let minus = target - array[i];
        //     if map.contains_key(&minus) {
        //         let index = map.get(&minus).unwrap();
        //         return vec![i as i32, *index];
        //     }
        // }
        for i in 0..size {
            let num = target - array[i];
            if !map.contains_key(&num) {
                map.insert(array[i], i as i32);
            }else {
                let index = map.get(&array[i]).unwrap();
                return vec![i as i32, *index];
            }
        }
        vec![]
    }

    // O(n^2)
    pub fn violent_two_sum(array: Vec<i32>, target: i32) -> Vec<i32> {
        let size  = array.len();
        for i in 0..size {
            for j in i+1..size {
                if array[i] + array[j] == target {
                    return vec![i as i32, j as i32];    // as关键字进行安全的转换：e as U是有效的仅当e是T类型而且T能强转为U。
                    // see http://shouce.jb51.net/rust-book-chinese/content/Casting%20Between%20Types%20%e7%b1%bb%e5%9e%8b%e8%bd%ac%e6%8d%a2.html
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let arr = vec![2, 7, 11, 15];
        let arr2 = vec![2, 7, 11, 15];
        let target = 9;
        let ret = Solution::violent_two_sum(arr, target);
        assert_eq!(ret, vec![0, 1]);
        let ret2 = Solution::violent_two_sum(arr2, target);
        assert_eq!(ret2, vec![0, 1]);

    }
}