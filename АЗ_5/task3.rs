pub fn invert_tree(root: Node) -> Node {
    if let Some(node) = root.clone() {
        let mut n = node.borrow_mut();
        let left = invert_tree(n.left.clone());
        let right = invert_tree(n.right.clone());
        n.left = right;
        n.right = left;
    }
    root
}
