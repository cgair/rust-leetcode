/**
 * [297] Serialize and Deserialize Binary Tree
 *
 * Design an algorithm to serialize and deserialize a binary tree. 
 * There is no restriction on how your serialization/deserialization algorithm should work. 
 * You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.
 *
 * 
 * Example:
 *
 *
 * Input: root = [1,2,3,null,null,4,5]
 * Output: [1,2,3,null,null,4,5]
 *
 *
 */

// problem: https://leetcode.cn/problems/serialize-and-deserialize-binary-tree/
use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

const NULL: &'static str = "#";

struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode<i32>>>>) -> String {
        if !root.is_some() {
            return String::from("");
        }
        let mut ret = String::from("");
        let mut stack = Vec::new();
        stack.push(root);
        while !stack.is_empty() {
            if let Some(maybe_root) = stack.pop() {
                match maybe_root {
                    Some(root) => {
                        ret.push_str(&root.borrow().val.to_string());
                        stack.push(root.borrow_mut().right.take());
                        stack.push(root.borrow().left.clone());
                    },
                    _ => {
                        ret.push_str(NULL);
                    }
                }
            }
        }

        ret
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        if data.is_empty() {
            return None;
        }
        let mut s_vec = data.chars().collect::<Vec<_>>();
        let mut index = 1usize;
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(s_vec[0].to_string().parse::<i32>().unwrap()))));
        root.as_mut().unwrap().borrow_mut().left = Self::build_tree(&s_vec, &mut index);
        root.as_mut().unwrap().borrow_mut().right = Self::build_tree(&s_vec, &mut index);

        root
    }

    fn build_tree(data: &[char], index: &mut usize) -> Option<Rc<RefCell<TreeNode<i32>>>> {
        if data[*index].to_string() == NULL {
            *index += 1;
            return None;
        } else {
            let mut node = Some(Rc::new(RefCell::new(TreeNode::new(data[*index].to_string().parse::<i32>().unwrap()))));
            *index += 1;
            node.as_mut().unwrap().borrow_mut().left = Self::build_tree(&data, index);
            node.as_mut().unwrap().borrow_mut().right = Self::build_tree(&data, index);
            node
        }
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

use crate::tree;
#[test]
fn test_297() {
    let obj = Codec::new();
    let se = obj.serialize(tree![1,2,3,null,null,4,5]);
    println!("{:?}", se);
    let de = obj.deserialize(se);
    println!("{:?}", de);
}
