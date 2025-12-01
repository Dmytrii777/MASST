#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

pub fn inorder_traversal(root: Option<Box<TreeNode>>) -> Vec<i32> {
    fn dfs(node: &Option<Box<TreeNode>>, out: &mut Vec<i32>) {
        if let Some(n) = node {
            dfs(&n.left, out);
            out.push(n.val);
            dfs(&n.right, out);
        }
    }

    let mut res = Vec::new();
    dfs(&root, &mut res);
    res
}
