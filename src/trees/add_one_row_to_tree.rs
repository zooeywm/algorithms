// Definition for a binary tree node.
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(unused)]
fn add_one_row(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
    depth: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
        currdepth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node_ref = root?;
        let mut node = node_ref.borrow_mut();
        if depth == 1 {
            let new_root = Rc::new(RefCell::new(TreeNode::new(val)));
            new_root.borrow_mut().left = Some(node_ref.clone());
            return Some(new_root);
        }

        if currdepth == depth - 1 {
            let leftman = node.left.take();
            let rightman = node.right.take();

            let left_node = Rc::new(RefCell::new(TreeNode::new(val)));
            let right_node = Rc::new(RefCell::new(TreeNode::new(val)));

            left_node.borrow_mut().left = leftman;
            left_node.borrow_mut().right = None;
            node.left = Some(left_node);

            right_node.borrow_mut().left = None;
            right_node.borrow_mut().right = rightman;
            node.right = Some(right_node);

            return Some(node_ref.clone());
        }

        node.left = helper(node.left.take(), val, depth, currdepth + 1);
        node.right = helper(node.right.take(), val, depth, currdepth + 1);

        Some(node_ref.clone())
    }

    helper(root, val, depth, 1)
}
