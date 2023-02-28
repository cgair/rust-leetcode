/**
 * [652] Find Duplicate Subtrees
 *
 * Given the root of a binary tree, return all duplicate subtrees.
 * For each kind of duplicate subtrees, you only need to return the root node of any one of them.
 * Two trees are duplicate if they have the same structure with the same node values.
 *
 * Example1:
 *
 *
 * Input: root = [1,2,3,4,null,2,4,null,null,4]
 * Output: [[2,4],[4]]
 * 
 * 
 * Example2:
 * 
 * 
 * Input: root = [2,1,1]
 * Output: [[1]]
 *
 * 
 * Example3:
 * 
 * 
 * Input: root = [2,2,2,3,null,3,null]
 * Output: [[2,3],[3]]
 */

// problem: https://leetcode.cn/problems/find-duplicate-subtrees/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::{TreeNode, to_tree};

pub struct Solution;
impl Solution {
    pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Vec<Option<Rc<RefCell<TreeNode<i32>>>>> {
        if root.is_none() { return vec![]; }

        let mut ret = Vec::new();
        let mut map = HashMap::new();   // Record all subtree and their occurrence times

        Solution::traverse(root, &mut map, &mut ret);

        ret
    }


    fn traverse(mut root: Option<Rc<RefCell<TreeNode<i32>>>>, map: &mut HashMap<String, usize>, ret: &mut Vec<Option<Rc<RefCell<TreeNode<i32>>>>>) -> String {
        if root.is_none() { return "#".to_string(); }

        let left = Solution::traverse(root.as_mut().unwrap().borrow_mut().left.clone(), map, ret);
        let right = Solution::traverse(root.as_mut().unwrap().borrow_mut().right.clone(), map, ret);
    
        /* 后序位置 */
        // 将左右子树和 node 节点序列化成字符串
        let mut subtree = String::from("");
        subtree.push_str(&left);
        subtree.push(',');
        subtree.push_str(&right);
        subtree.push(',');
        subtree.push_str(&root.as_ref().unwrap().borrow().val.to_string());
        
        if let Some(&times) = map.get(&subtree) {
            if times == 1 { ret.push(root); }
        }
        map.entry(subtree.clone()).and_modify(|counter| *counter += 1).or_insert(1);

        println!("{subtree:}");

        // *map.entry(subtree.clone()).or_insert(0) += 1;
        // if let Some(&times) = map.get(&subtree) {
        //     if times == 2 { ret.push(root); }
        // }

        subtree
    }
}


#[test]
fn test_652() {
    let (a, b): (Option<i32>, Option<i32>) = (None, None);
    assert_eq!(a, b);

    assert_eq!(
        Solution::find_duplicate_subtrees(to_tree(vec![Some(1), Some(2), Some(3), Some(4), None, Some(2), Some(4), None, None, Some(4)])),
        vec![to_tree(vec![Some(4)]), to_tree(vec![Some(2), Some(4)]),]
    )
}

