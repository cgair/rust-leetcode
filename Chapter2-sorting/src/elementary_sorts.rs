/**
 *  In this section, we shall study two elementary sorting methods (selection sort and insertion sort) and a variation of one of them (shellsort).
 */

// /////////////////////// //
//      Selection sort     //
// /////////////////////// //
// Repeatedly selecting the smallest remaining item.

// First, find the smallest item in the array, and exchange it with the first entry. 
// Then, find the next smallest item and exchange it with the second entry. 
// Continue in this way until the entire array is sorted.

pub fn selection_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        // let mut curr_smallest = arr[i];
        // let mut index = i;  // 这里我既记录最小的值, 又记录最小的位置, 实际上只需要记录最小的位置.
        let mut smallest = i;
        // for j in (i + 1)..len {
        //     if arr[j] < curr_smallest {
        //         curr_smallest = arr[j];
        //         index = j;
        //     }
        // }
        // arr[index] = arr[i];
        // arr[i] = curr_smallest;

        for j in (i + 1)..len {
            if arr[j] < arr[smallest] {
                smallest = j;
            }
        }
        
        arr.swap(i, smallest);
    }
}
// 复杂度分析: 第一次内循环比较 N - 1 次， 然后是 N-2 次， N-3 次, ..., 1 次.
// 共比较的次数是 (N - 1) + (N - 2) + ... + 1, 由等差数列和 (Sn = n * a1 + n(n-1)d / 2 或 Sn = n(a1+an) / 2) 
// 得 (N - 1 + 1) * N / 2 = N^2 / 2 -> 其时间复杂度为 O(N^2)

// <https://matklad.github.io/2021/05/31/how-to-test.html>
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_selection_sort() {
        println!("Sort numbers ascending");
        let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        println!("Before: {:?}", numbers);
        selection_sort(&mut numbers);
        println!("After:  {:?}\n", numbers);
        assert_eq!(numbers, [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);

        println!("Sort strings alphabetically");
        let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
        println!("Before: {:?}", strings);
        selection_sort(&mut strings);
        println!("After:  {:?}\n", strings);
    }
}