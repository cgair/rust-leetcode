/**
 * 
 * Quicksort works by obtaining a pivot and partitioning the list around it 
 * (bigger elements on one side and smaller elements on the other side) until everything is sorted. 
 * The method falls under divide and conquer technique.
*/
pub fn quick_sort<T: Ord + Clone>(nums: &mut [T]) {
    _quick_sort(nums, 0, nums.len() as isize - 1);
}

fn _quick_sort<T: Ord + Clone>(nums: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = partition(nums, low, high);
        _quick_sort(nums, low, p - 1);
        _quick_sort(nums, p + 1, high);
    }
}

fn partition<T: Ord + Clone>(nums: &mut [T], low: isize, high: isize) -> isize {
    let pivot = nums[low as usize].clone();  // Select the first element as pivot
    let (mut left, mut right) = (low, high);
    while left < right {
        while left < right && nums[right as usize] >= pivot { right -= 1; }
        nums.swap(left as usize, right as usize);
        // nums[left as usize] = nums[right as usize].clone();
        while left < right && nums[left as usize] <= pivot { left += 1; }
        nums.swap(left as usize, right as usize);
        // nums[right as usize] = nums[left as usize].clone();
    }
    nums[left as usize] = pivot;

    left    
}

// 使用数组模拟栈的非递归实现:
fn non_recursive_quick_sort<T: Ord + Clone>(nums: &mut [T]) {
    if nums.len() <= 1 { return; }
    // 栈 中保存下次需要排序的子数组的开始位置和结束位置 (i.e., pair (start, end))
    let mut stack = Vec::with_capacity(2000); // 假设递归不超过 1000 层

    stack.push((0isize, nums.len() as isize - 1));
    
    while !stack.is_empty() {
        if let Some((lo, hi)) = stack.pop() {
            let p = partition(nums, lo, hi);
            if (p + 1 < hi) { // 右边子数组索引入栈
                stack.push((p + 1, hi));
            }
            if (p - 1 > lo) {
                stack.push((lo, p - 1))
            }
        }
    }
}



#[test]
fn test_quick_sort() {
    use std::time::Instant;
    println!("===========================Sort numbers ascending===========================");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("              Before: {:?}", numbers);
    let start = Instant::now();
    quick_sort(&mut numbers);
    println!("    [recursive]After: {:?}, time: {:?}\n", numbers, start.elapsed());
    let start = Instant::now();
    non_recursive_quick_sort(&mut numbers);
    println!("[non recursive]After: {:?}, time: {:?}\n", numbers, start.elapsed());
}