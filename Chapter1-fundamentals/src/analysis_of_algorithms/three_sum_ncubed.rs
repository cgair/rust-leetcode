//! A program with cubic running time. Reads n integers
//! and counts the number of triples that sum to exactly 0
//! (ignoring integer overflow).

pub struct ThreeSum;

impl ThreeSum {
    /// Calculate the number of triples (i, j, k) with {i < j < k}
    /// such that {a[i] + a[j] + a[k] == 0}.
    /// 
    pub fn count(&self, arr: &[i32]) -> usize {
        let mut count = 0;
        let len = arr.len();
        for i in 0..len {
            for j in i..len {
                for k in j..len {
                    if arr[i] + arr[j] + arr[k] == 0 { count += 1; }
                }
            }
        }

        count
    }

    /// Prints to standard output the (i, j, k) with {i < j < k}
    /// such that {a[i] + a[j] + a[k] == 0}.
    /// 
    pub fn print_all(&self, arr: &[i32]) {
        let len = arr.len();
        for i in 0..len {
            for j in i..len {
                for k in j..len {
                    if arr[i] + arr[j] + arr[k] == 0 {
                        println!("a[{}] + a[{}] + a[{}] = 0", i, j, k);
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::SUM;
    use std::time::Instant;
    use std::io::{BufReader, BufRead};
    use std::fs;
    use std::str::FromStr;

    #[test]
    fn test_ncubed_three_sum() {
        let ts = ThreeSum;
        let mut arr = Vec::new();
        let file = fs::File::open("/Users/chenge/Desktop/CGs/Local-workplace/rust-leetcode/fundamentals/data/sum/1Kints.txt").unwrap();
        let lines = BufReader::new(file).lines();
        for line in lines {
            let num_str = line.unwrap();
            let num_str = num_str.trim();
            arr.push(num_str.parse::<i32>().unwrap());
        }
        let start = Instant::now();
        let sum = ts.count(&arr);
        println!("three sum: {}, elapsed time = {:?}", sum, start.elapsed());
    }

    #[test]
    fn test_ncubed_three_sum_all() {
        let ts = ThreeSum;
        for path in SUM.iter() {
            let mut arr = Vec::new();
            let file = fs::File::open(path).unwrap();
            println!("reading: {}", path.display());
            let lines = BufReader::new(file).lines();
            for line in lines {
                let num_str = line.unwrap();
                let num_str = num_str.trim();
                arr.push(num_str.parse::<i32>().unwrap());
            }
            let start = Instant::now();
            let sum = ts.count(&arr);
            println!("three sum: {}, elapsed time = {:?}", sum, start.elapsed());
        }
    }
}