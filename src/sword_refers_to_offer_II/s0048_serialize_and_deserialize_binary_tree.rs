/**
 * [剑指 Offer II 48] 序列化与反序列化二叉树
 *
 * 序列化是将一个数据结构或者对象转换为连续的比特位的操作, 进而可以将转换后的数据存储在一个文件或者内存中, 同时也可以通过网络传输到另一个计算机环境, 采取相反方式重构得到原数据.
 * 请设计一个算法来实现二叉树的序列化与反序列化. 
 * 这里不限定你的序列/反序列化算法执行逻辑, 只需要保证一个二叉树可以被序列化为一个字符串并且将这个字符串反序列化为原始的树结构.
 * 
 * 
 * 示例 1:
 * 输入: root = [1,2,3,null,null,4,5]
 * 输出: [1,2,3,null,null,4,5]
 * 
 * 
 * 示例 2:
 * 输入: root = [1]
 * 输出: []
 * 
 * 
 * 示例 3:
 * 输入: root = [1]
 * 输出: [1]
 *
 */

use std::rc::Rc;
use std::cell::RefCell;
use super::TreeNode;

const NULL: &'static str = "#";

struct Codec { }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec { }
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() { return String::from(""); }
        let mut serialize = String::new();
        
        self.preorder_traversal(&root, &mut serialize);
        serialize
    }

    fn serialize2(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() { return String::from(""); }
        let mut serialize = String::new();
        
        self.preorder_traversal2(root, &mut serialize);
        serialize
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() { return None; }

        let mut s = data.chars().rev().collect::<Vec<_>>();

        self.construct_tree(&mut s)
    }

    fn preorder_traversal(&self, root: &Option<Rc<RefCell<TreeNode>>>, serialize: &mut String) {
        if root.is_none() {
            serialize.push_str(NULL);
            return;
        }
        serialize.push_str(&root.as_ref().unwrap().borrow().val.to_string());
        self.preorder_traversal(&root.as_ref().unwrap().borrow().left, serialize);
        self.preorder_traversal(&root.as_ref().unwrap().borrow().right, serialize);
    }

    fn preorder_traversal2(&self, root: Option<Rc<RefCell<TreeNode>>>, serialize: &mut String) {
        if root.is_none() {
            serialize.push_str(NULL);
            return;
        }
        serialize.push_str(&root.as_ref().unwrap().borrow().val.to_string());
        self.preorder_traversal2(root.as_ref().unwrap().borrow().left.clone(), serialize);
        self.preorder_traversal2(root.as_ref().unwrap().borrow().right.clone(), serialize);
    }

    fn construct_tree(&self, s: &mut Vec<char>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = None;
        if let Some(ch) = s.pop() {
            match ch {
                '#' => { return None; },
                _ => {
                    // let val = ch as i32 - 0x30;
                    let val = ch.to_string().parse::<i32>().unwrap();
                    root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    root.as_mut().unwrap().borrow_mut().left = self.construct_tree(s);
                    root.as_mut().unwrap().borrow_mut().right = self.construct_tree(s);
                }
            }
        }
        return root;
    }
}



use crate::tree;
#[test]
fn test_swordII_48() {
    use std::time::Instant;
    let obj = Codec::new();
    // TODO(cgair): What's behind this?
    let start = Instant::now();
    let se = obj.serialize(tree![1,2,3,null,null,4,5]);
    println!("serialize(): {:?}", start.elapsed());

    let start = Instant::now();
    let se = obj.serialize2(tree![1,2,3,null,null,4,5]);
    println!("serialize2(): {:?}", start.elapsed());

    println!("{:?}", se);
    let de = obj.deserialize(se);
    println!("{:?}", de);
    assert_eq!(
        de,
        tree![1,2,3,null,null,4,5]
    );

    let se = obj.serialize(tree![4,-7,-3,null,null,-9,-3,9,-7,-4,null,6,null,-6,-6,null,null,0,6,5,null,9,null,null,-1,-4,null,null,null,-2]);
    println!("{:?}", se);
    let de = obj.deserialize(se);
    assert_eq!(
        de,
        tree![4,-7,-3,null,null,-9,-3,9,-7,-4,null,6,null,-6,-6,null,null,0,6,5,null,9,null,null,-1,-4,null,null,null,-2]
    );
}

