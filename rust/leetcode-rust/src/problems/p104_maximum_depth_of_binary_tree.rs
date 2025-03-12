use std::cell::RefCell;
use std::rc::Rc;

use crate::util::treenode_leetcode::TreeNode;


pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(root) => {
            let root_node_ref = root.borrow();
            match (&root_node_ref.left, &root_node_ref.right) {
                (None, None) => return 1,
                (None, Some(right)) => 1 + max_depth(Some(right.clone())),
                (Some(left), None) => 1 + max_depth(Some(left.clone())),
                (Some(left), Some(right)) => {
                    1 + max_depth(Some(right.clone())).max(max_depth(Some(left.clone())))
                }
            }
        }
        None => return 0,
    }
}

// impl Solution {

//     }
// }
