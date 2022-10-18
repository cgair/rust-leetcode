/*
Quicksort works by obtaining a pivot and partitioning the list around it (bigger elements on one side and smaller elements on the other side) until everything is sorted. 
*/
pub fn quick_sort(nums: &mut [usize]) {
    
}



#[test]
fn test_quick_sort() {
    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    // quick_sort(&mut numbers);
    println!("After:  {:?}\n", numbers);
}