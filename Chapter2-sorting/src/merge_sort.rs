/**
 * The algorithms that we consider in this section is based on a simple operation known as merging: 
 *     combining two ordered arrays to make one larger ordered array. 
 * 
 * This operation immediately lends itself to a simple recursive sort method known as mergesort: 
 *     to sort an array, divide it into two halves, sort the two halves (recursively), and then merge the results.
 */



// /////////////////////// //
//       Merge sort        //
// /////////////////////// //

// Mergesort guarantees to sort an array of N items in time proportional to N log N, no matter what the input. 
// Its prime disadvantage is that it uses extra space proportional to N.

/// Rearranges the array in ascending order, using
/// [merge sort](https://en.wikipedia.org/wiki/Merge_sort).
/// 
pub fn merge_sort<T: Copy + PartialOrd + Default>(arr: &mut [T]) {
    let len = arr.len();
    // let mut aux = Vec::with_capacity(len);
    let mut aux = vec![T::default(); len];
    sort(arr, &mut aux, 0, len - 1);

}

/// Mergesort a[lo..hi] using auxiliary array aux[lo..=hi]
fn sort<T: Copy + PartialOrd>(arr: &mut [T], aux: &mut [T], lo: usize, hi: usize) {
    if lo >= hi { return; }

    let mid = (lo + hi) / 2;

    sort(arr, aux, lo, mid);     /* 递归将 arr[lo..=mid] 归并为有序的 aux[lo..=mid] */
    sort(arr, aux, mid + 1, hi); /* 递归将 arr[mid + 1 ..=hi] 归并为有序的 aux[mid + 1..=hi] */
    merge(arr, aux, lo, mid, hi);
}

/// Stably merge a[lo .. mid] with a[mid+1 ..hi] using aux[lo .. hi]
fn merge<T: Copy + PartialOrd>(arr: &mut [T], aux: &mut [T], lo: usize, mid: usize, hi: usize) {
    // precondition: a[lo .. mid] and a[mid + 1 .. hi] are sorted subarrays

    /* This is more C*/
    let mut i = lo;  // 第一个数组索引, 指向第一个元素.
    let mut j = mid + 1;  // 第二个数组索引, 指向第一个元素.

    for t in 0..=hi-lo {    // 临时数组索引, 指向第一个元素.
        // 若 i 到达第一个数组的尾部, 将第二个数组余下元素复制到 临时数组中
        if i == mid + 1 {
           aux[t] = arr[j];
           j += 1;
           continue;
        }
        // 若 j 到达第二个数组的尾部, 将第一个数组余下元素复制到 临时数组中
        if j == hi + 1 {
            aux[t] = arr[i];
            i += 1;
            continue;
        }
        // 如果第一个数组的当前元素 比 第二个数组的当前元素小,将 第一个数组的当前元素复制到 临时数组中
        if arr[i] < arr[j] {
            aux[t] = arr[i];
            i += 1
        } else {
            aux[t] = arr[j];
            j += 1;
        }
    }

    // merge back to arr[]
    // i = lo (被排序的数组 arr 的起始位置)
    // j = 0 (临时数组的起始位置)
    arr[lo..=hi].copy_from_slice(&aux[0..=hi - lo]);

    // postcondition: a[lo .. hi] is sorted
}

#[test]
fn test_merge_sort() {
    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    merge_sort(&mut numbers);
    println!("After:  {:?}\n", numbers);
    assert_eq!(numbers, [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);

    let mut numbers = vec![5, 1, 9, 3, 7, 4, 8, 6, 2];
    println!("Before: {:?}", numbers);
    merge_sort(&mut numbers);
    println!("After:  {:?}\n", numbers);
    assert_eq!(numbers, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
}


#[test]
fn remember_slice_index() {
    let value = vec![0, 1, 2, 3, 4, 5, 6];
    assert_eq!(
        &value[0..7],
        &value
    );
    assert_eq!(
        &value[0..=6],
        &value
    );
    assert_eq!(
        &value[0..3],
        &[0, 1, 2]
    )
}