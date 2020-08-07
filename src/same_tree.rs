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

/// 相同的树
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/same-tree/)
///
/// ```text
/// 给定两个二叉树，编写一个函数来检验它们是否相同。
/// 如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。
///
/// 示例1:
/// 输入:       1         1
///           / \       / \
///          2   3     2   3
///
///         [1,2,3],   [1,2,3]
///
/// 输出: true
///
/// 示例 2:
/// 输入:      1          1
///           /           \
///          2             2
///
///         [1,2],     [1,null,2]
///
/// 输出: false
///
/// 示例3:
/// 输入:       1         1
///           / \       / \
///          2   1     1   2
///
///         [1,2,1],   [1,1,2]
///
/// 输出: false
/// ```
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution {}
impl Solution {
    /// 深度优先搜索
    pub fn dfs(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val
                    && Solution::dfs(p.left.clone(), q.left.clone())
                    && Solution::dfs(p.right.clone(), q.right.clone())
            }
            (Some(_), None) => false,
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TreeNode 宏
    macro_rules! tree {
        () => {
            None
        };

        ($val:expr) => {
            Some(Rc::new(RefCell::new(TreeNode::new($val))))
        };

        ($val:expr, $left:expr, $right:expr) => {{
            let mut t = TreeNode::new($val);
            t.left = $left;
            t.right = $right;

            Some(Rc::new(RefCell::new(t)))
        }};
    }

    #[test]
    fn dp() {
        assert_eq!(
            Solution::dfs(tree!(1, tree!(2), tree!(3)), tree!(1, tree!(2), tree!(3))),
            true
        );
        assert_eq!(
            Solution::dfs(tree!(1, tree!(2), None), tree!(1, None, tree!(2))),
            false
        );
        assert_eq!(
            Solution::dfs(tree!(1, tree!(2), tree!(1)), tree!(1, tree!(1), tree!(2))),
            false
        );
    }
}
