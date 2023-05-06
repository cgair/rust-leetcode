/**
 * 找到当前数字序列中最大 (最小) 的数,记录其所在位置.
 * 将其和最前面 (最后面) 的数进行交换, 使最小 (最大) 的元素上浮 (下沉) 到本次排序的最前面 (最后面), 完成一趟 (pass) 排序.
 * 下一趟排序时, 已经有序的元素不再参与: 这样的话，n 个元素需要进行 n-1 趟排序
 */

// 复杂度分析: 第一次内循环比较 N - 1 次， 然后是 N-2 次， N-3 次, ..., 1 次.
// 共比较的次数是 (N - 1) + (N - 2) + ... + 1, 由等差数列和 (Sn = n * a1 + n(n-1)d / 2 或 Sn = n(a1+an) / 2) 
// 得 (N - 1 + 1) * N / 2 = N^2 / 2 -> 其时间复杂度为 O(N^2)

/// Sorts a slice in-place using
/// [selction sort](https://en.wikipedia.org/wiki/Selection_sort).
///
fn selection<T: PartialOrd>(nums: &mut [T]) {
    let len = nums.len();
    for i in 0..len - 1 {
        let mut smallest = i;
        for j in i + 1..len {
            if nums[smallest] > nums[j] {
                // nums.swap(i, j);    // 额外交换了
                smallest = j;
            }
        }
        nums.swap(smallest, i);
    }
}

#[test]
fn test_selection_sort() {
    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    selection(&mut numbers);
    println!("After:  {:?}\n", numbers);
    assert_eq!(numbers, [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782]);

    println!("Sort strings alphabetically");
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", strings);
    selection(&mut strings);
    println!("After:  {:?}\n", strings);
}
