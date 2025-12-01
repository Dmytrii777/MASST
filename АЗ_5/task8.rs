use std::collections::{BTreeMap, BinaryHeap};

pub fn vertical_traversal(root: Node) -> Vec<Vec<i32>> {
    let mut map: BTreeMap<i32, Vec<(i32,i32)>> = BTreeMap::new();

    fn dfs(n: &Node, row: i32, col: i32, m: &mut BTreeMap<i32, Vec<(i32,i32)>>) {
        if let Some(node) = n {
            let b = node.borrow();
            m.entry(col).or_default().push((row, b.val));
            dfs(&b.left, row+1, col-1, m);
            dfs(&b.right, row+1, col+1, m);
        }
    }

    dfs(&root, 0, 0, &mut map);

    let mut res = vec![];
    for (_col, mut v) in map {
        v.sort(); // спочатку row, потім value
        res.push(v.into_iter().map(|(_, val)| val).collect());
    }
    res
}
