pub fn max_path_sum(root: Node) -> i32 {
    fn dfs(n: &Node, best: &mut i32) -> i32 {
        if let Some(x) = n {
            let b = x.borrow();
            let left = dfs(&b.left, best).max(0);
            let right = dfs(&b.right, best).max(0);
            *best = (*best).max(b.val + left + right);
            b.val + left.max(right)
        } else {
            0
        }
    }
    let mut ans = i32::MIN;
    dfs(&root, &mut ans);
    ans
}
