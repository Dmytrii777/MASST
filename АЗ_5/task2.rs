pub fn is_symmetric(root: Node) -> bool {
    fn mirror(a: &Node, b: &Node) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(x), Some(y)) => {
                let x = x.borrow();
                let y = y.borrow();
                x.val == y.val
                    && mirror(&x.left, &y.right)
                    && mirror(&x.right, &y.left)
            }
            _ => false,
        }
    }
    mirror(&root, &root)
}
