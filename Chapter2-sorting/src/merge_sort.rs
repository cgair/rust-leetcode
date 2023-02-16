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
pub fn merge_sort<T: Copy>(arr: &mut [T]) {
    let len = arr.len();
    let mut aux = Vec::with_capacity(len);
    sort(arr, &mut aux, 0, len - 1);

}

/// Mergesort a[lo..hi] using auxiliary array aux[lo..=hi]
fn sort<T: Copy>(arr: &mut [T], aux: &mut [T], lo: usize, hi: usize) {
    if lo >= hi { return; }

    let mid = (lo + hi) / 2;

    sort(arr, aux, lo, mid);     /* 递归将 arr[lo..=mid] 归并为有序的 aux[lo..=mid] */
    sort(arr, aux, mid + 1, hi); /* 递归将 arr[mid + 1 ..=hi] 归并为有序的 aux[mid + 1..=hi] */
    merge(arr, aux, lo, mid, hi);
}

/// Stably merge a[lo .. mid] with a[mid+1 ..hi] using aux[lo .. hi]
fn merge<T: Copy>(arr: &mut [T], aux: &mut [T], lo: usize, mid: usize, hi: usize) {
    // precondition: a[lo .. mid] and a[mid + 1 .. hi] are sorted subarrays

    // copy to aux[]
    for i in lo..=hi {
        aux[i] = arr[i];
    }

    // merge back to a[]


    // postcondition: a[lo .. hi] is sorted
}

#[test]
fn test_merge_sort() {
    let mut value = vec![5, 1, 9, 3, 7, 4, 8, 6, 2];
    merge_sort(&mut value);
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