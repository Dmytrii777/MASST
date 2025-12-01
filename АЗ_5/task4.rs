pub fn kth_smallest(root: Node, k: i32) -> i32 {
    fn inorder(node: &Node, out: &mut Vec<i32>) {
        if let Some(n) = node {
            let b = n.borrow();
            inorder(&b.left, out);
            out.push(b.val);
            inorder(&b.right, out);
        }
    }

    let mut arr = vec![];
    inorder(&root, &mut arr);
    arr[(k - 1) as usize]
}
