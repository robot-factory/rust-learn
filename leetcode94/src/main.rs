use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// impl Solution {
//     pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
//         let mut res: Vec<i32> = Vec::new();
//         let mut stack = Vec::new();
//         let mut r = root.clone();
//         while r.is_some() || !stack.is_empty() {
//             while let Some(node) = r {
//                 stack.push(node.clone());
//                 r = node.borrow().left.clone();
//             }
//             r = stack.pop();
//             if let Some(node) = r {
//                 res.push(node.borrow().val);
//                 r = node.borrow().right.clone();
//             }
//         }
//         res
//     }
// }

fn main() {
    let mut v1 = vec![];
    v1.push(1);
    v1.push(2);
    let item =if let Some(x) = v1.pop() {
        x
    } else {
        0
    };
    println!("{}", item);
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut p = root.clone();
    let mut stack = Vec:new();
    let mut result = vec![];
    while let p.is_some() || !stack.is_empty() {
        while Some(node) = p {
            stack.push(node.clone());
            p =node.borrow().left.clone();
        }
        p = stack.pop();
        if let Some(node) = p {
            result.push(node.borrow().val);
            p = node.borrow().right.clone();
        }
    }
    result
}