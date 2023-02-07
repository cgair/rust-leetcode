//! Solves the 3-sum problem in time proportional to N^2 log N time.

pub struct ThreeSumFast;

impl ThreeSumFast {
    /// Calculate the number of triples (i, j, k) with {i < j < k}
    /// such that {a[i] + a[j] + a[k] == 0}.
    /// 
    pub fn count(&self, arr: &mut [i32]) -> usize {
        let len = arr.len();
        let mut count = 0;
        arr.sort();
        for i in 0..len {
            for j in i..len {
                let search = 0 - arr[i] - arr[j];
                if arr.binary_search(&search).is_ok() {
                    count += 1;
                }
            }
        }
        count
    }
}

use std::fs;
use std::io::{BufReader, BufRead};
use std::time::Instant;

#[test]
fn test_nsquarelogn_three_sum() {
    let ts = ThreeSumFast;
    let mut arr = Vec::new();
    let file = fs::File::open("/Users/chenge/Desktop/CGs/Local-workplace/rust-leetcode/fundamentals/data/sum/1Kints.txt").unwrap();
    let lines = BufReader::new(file).lines();
    for line in lines {
        let num_str = line.unwrap();
        let num_str = num_str.trim();
        arr.push(num_str.parse::<i32>().unwrap());
    }
    let start = Instant::now();
    let sum = ts.count(&mut arr);
    println!("three sum: {}, elapsed time = {:?}", sum, start.elapsed());
    
}
