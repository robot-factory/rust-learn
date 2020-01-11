use std::cell::RefCell;
use std::rc::Rc;

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
        right: None
        }
    }
}

fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    generate_trees_impl(1, n)
}

fn generate_trees_impl(l: i32, r: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if l > r {
        vec![None]
    } else {
        let mut res = vec![];
        for i in l..r+1 {
            let lnodes = generate_trees_impl(l, i-1);
            let rnodes = generate_trees_impl(i+1, r);
            for ln in lnodes.iter() {
                for rn in rnodes.iter() {
                    let node = Some(Rc::new(RefCell::new(TreeNode{
                        val: i,
                        left: ln.clone(),
                        right:rn.clone(),
                    }))); 
                    // node.unwrap().clone().borrow_mut().left = ln.clone();
                    // node.unwrap().borrow_mut().right = rn.clone();
                    res.push(node);
                }
            }
        }
        res
    }
}

fn main() {
    let trees = generate_trees(3);
    // println!("{:?}", trees);
}
