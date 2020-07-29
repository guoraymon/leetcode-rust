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

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

/// 二叉树的最大深度
///
/// 来源：[LeetCode](https://leetcode-cn.com/problems/maximum-depth-of-binary-tree/)
///
/// ```text
/// 给定一个二叉树，找出其最大深度。
///
/// 二叉树的深度为根节点到最远叶子节点的最长路径上的节点数。
///
/// 说明: 叶子节点是指没有子节点的节点。
///
/// 示例：
/// 给定二叉树 [3,9,20,null,null,15,7]，
///     3
///    / \
///   9  20
///     /  \
///    15   7
/// 返回它的最大深度 3 。
/// ```
pub struct Solution {}

impl Solution {
    /// 递归
    /// ```text
    /// 时间复杂度：O(n)，其中 n 为二叉树节点的个数。每个节点在递归中只被遍历一次。
    /// 空间复杂度：O(height)，其中 height 表示二叉树的高度。递归函数需要栈空间，而栈空间取决于递归的深度，因此空间复杂度等价于二叉树的高度。
    /// ```
    pub fn recursion(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return match root {
            Some(root) => {
                let root = root.borrow();
                max(
                    Self::recursion(root.left.clone()),
                    Self::recursion(root.right.clone()),
                ) + 1
            }
            None => 0,
        };
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
    fn recursion() {
        assert_eq!(
            Solution::recursion(tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)))),
            3
        );
    }
}
