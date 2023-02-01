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

/// Sorts a slice in-place using
/// [selction sort](https://en.wikipedia.org/wiki/Selection_sort).
///
pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
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


// /////////////////////// //
//      Insertion sort     //
// /////////////////////// //

// Insertion sort works by searching the list sequentially and 
// moving the unsorted items into a sorted sublist on the left side of the list. 

/// Sorts a slice in-place using
/// [insertion sort](https://en.wikipedia.org/wiki/Insertion_sort).
///
/// This sorting algorithm is very efficient when used on small data sets.
/// This is because insertion sort has constant space complexity and works
/// very fast when used on partially sorted data.
///
pub fn insertion_sort<T: PartialOrd> (arr: &mut [T]) {
    let len = arr.len();

    // assert!(len > 2);
    // let mut right = 1usize;
    // while right < len {
    //     for i in 0..right {
    //         if arr[right] <= arr[i] {
    //             arr.swap(i, right);
    //             for j in (i + 1)..right {
    //                 arr.swap(j, right);
    //             }
    //             break;
    //         }
    //     }
    //     right += 1;
    // }

    for i in 1..len {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }

    }
}
// 复杂度分析: For randomly ordered arrays of length N with with distinct keys, 
// insertion sort uses ~N2/4 compares and ~N2/4 exchanges on the average. 
// The worst case is ~ N2/2 compares and ~ N2/2 exchanges and the best case is N-1 compares and 0 exchanges.


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

    #[test]
    fn test_insertion_sort() {
        println!("Sort numbers ascending");
        let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
        println!("Before: {:?}", numbers);
        insertion_sort(&mut numbers);
        println!("After:  {:?}\n", numbers);
        assert_eq!(numbers, [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);
    
        println!("Sort strings alphabetically");
        let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
        println!("Before: {:?}", strings);
        insertion_sort(&mut strings);
        println!("After:  {:?}\n", strings);
        assert_eq!(strings, ["airplane", "art", "beach", "car", "hotel", "house"]);
    }
}