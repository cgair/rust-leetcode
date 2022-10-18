/**
 * [370] Range Addition
 *
 *
 */

struct NumArray {
    inner: Vec<i32>,
    diff: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let length = nums.len();
        assert!(length > 0);
        let mut diff = Vec::with_capacity(length);
        let inner = nums.clone();
        diff.push(nums[0]);
        for i in 1..length {
            let d = nums[i] - nums[i - 1];
            diff.push(d);
        }

        Self { 
            inner,
            diff
        }
    }
    
    fn increment(&mut self, left: i32, right: i32, val: i32) {
        assert!(left <= right, "left <= right");
        self.diff[left as usize] += val;
        let next = (right + 1) as usize;
        if next < self.diff.len() {
            self.diff[(right + 1) as usize] -= val;
        }
    }

    fn restore(&self) -> Vec<i32> {
        let length = self.inner.len();
        let mut ret = Vec::with_capacity(length);
        ret.push(self.diff[0]);
        for i in 1..length {
            let s = ret[i - 1] + self.diff[i];
            ret.push(s);
        }

        ret
    }
}

/*
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_370() {
        let mut nums = NumArray::new(vec![0, 0, 0, 0, 0]);
        let ops = vec![[1, 3, 2], [2, 4, 3], [0, 2, -2]];
        for o in ops {
            nums.increment(o[0], o[1], o[2]);
        }
        let ret = nums.restore();
        assert_eq!(ret, vec![-2, 0, 3, 5, 3]);
    }
}