// TreeNode 宏
#[macro_export]
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
