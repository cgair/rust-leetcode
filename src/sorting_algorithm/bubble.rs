/*
Bubble sorting simply iterates over the entire list and compare adjacent elements in the list, 
and after each comparison, place them in the right order of magnitude. 
This works by swapping adjacent items if they are not in the correct order. 
The process is repeated n-1 times for a list of n items. In each such iteration, the largest element is arranged in the end.
*/
// 适合小数据的排序, 但算法复杂度较高, 不适合大数据量. 
/*
复杂度:
    数据完全有序: O(n) -> 增加一个 swap (line 15) 的标志, 当前一轮没有进行交换时, 说明数组已经有序, 直接退出.
    其他情况下, 几乎总是 O(n2)
*/
pub fn bubble_sort<F: Fn(isize, isize) -> isize>(nums: &mut [isize], compare_fn: F) -> Vec<isize> {
    let length = nums.len();
    let mut swap = false;

    for i in 0..length {
        for j in 0..length - 1 - i {    // NOTE: **length - 1 - i**
            if compare_fn(nums[j], nums[j + 1]) > 0 {
                // let tmp = nums[j];
                // nums[j] = nums[j + 1];
                // nums[j + 1] = tmp;
                nums.swap(j, j + 1);
            }
            swap = true;
        }
        if swap == false {
            break;
        }
    }
    nums.to_vec()
}
// TODO:
// pub fn bubble_sort<T: Ord>(arr: &mut [T]) {}
// < T > indicates that this is a generic data type.
// The < T: Ord > is declaring this function to be generic over any type T that implements the Ord trait.

#[test]
fn test_bubble_sort() {
    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    bubble_sort(&mut numbers, compare);
    println!("After:  {:?}\n", numbers);
}

fn compare(a: isize, b: isize) -> isize{
    if a <= b {
        return 0;
    } else {
        return 1;
    }
}