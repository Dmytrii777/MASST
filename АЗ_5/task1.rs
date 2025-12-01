pub fn is_same_tree(p: Node, q: Node) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            let a = a.borrow();
            let b = b.borrow();
            a.val == b.val
                && is_same_tree(a.left.clone(), b.left.clone())
                && is_same_tree(a.right.clone(), b.right.clone())
        }
        _ => false,
    }
}
