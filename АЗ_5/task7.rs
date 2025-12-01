pub fn min_camera_cover(root: Node) -> i32 {
    fn dfs(n: &Node, cams: &mut i32) -> i32 {
        if n.is_none() { return 2; }
        let b = n.as_ref().unwrap().borrow();
        let l = dfs(&b.left, cams);
        let r = dfs(&b.right, cams);

        if l == 0 || r == 0 {
            *cams += 1;
            return 1;
        }
        if l == 1 || r == 1 {
            return 2;
        }
        0
    }

    let mut cams = 0;
    if dfs(&root, &mut cams) == 0 {
        cams += 1;
    }
    cams
}
