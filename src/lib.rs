mod data_structure;
mod s0001_two_sum;
mod s0002_add_two_numbers;
mod s0206_reverse_linked_list;
mod s0003_longest_substring_without_repeating_characters;
mod s0875_koko_eating_bananas;
mod s0303_range_sum_query_immutable;
mod s0304_range_sum_query_2d_immutable;
mod s0370_range_addition;
mod s0021_merge_two_sorted_lists;
mod s0019_remove_nth_node_from_end_of_list;
mod s0026_remove_duplicates_from_sorted_array;
mod s0027_remove_element;
mod s0088_merge_sorted_array;
mod s0023_merge_k_sorted_lists;
mod s0496_next_greater_element;
mod s0876_middle_of_the_linked_list;
mod s0005_longest_palindromic_substring;
mod s0144_binary_tree_preorder_traversal;
mod s0297_serialize_and_deserialize_binary_tree;
mod s0105_construct_binary_tree_from_preorder_and_inorder_traversal;

mod s0438_find_all_anagrams_in_a_string;
mod s0704_binary_search;
mod s0226_invert_binary_tree;
mod s0042_trapping_rain_water;
mod s0203_remove_linked_list_elements;
mod s0111_minimum_depth_of_binary_tree;

mod s0142_linked_list_cycle_II;

#[allow(non_snake_case)]
mod sword_refers_to_offer_II;
mod sorting_algorithm;

pub use s0206_reverse_linked_list::test_reverse_list;
pub use data_structure::{ListNode, TreeNode};

// for benchmark
#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

// helper function for test
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut curr = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = curr;
        curr = Some(Box::new(node));
    }
    curr
}

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());
    for children in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(v) = children[0] {
            parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }
        if children.len() > 1 {
            if let Some(v) = children[1] {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }

    head
}

#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            $crate::to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}