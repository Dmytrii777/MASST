pub struct Codec;

impl Codec {
    pub fn serialize(&self, root: Node) -> String {
        fn ser(n: &Node, s: &mut String) {
            if let Some(x) = n {
                let b = x.borrow();
                s.push_str(&b.val.to_string());
                s.push(',');
                ser(&b.left, s);
                ser(&b.right, s);
            } else {
                s.push_str("null,");
            }
        }
        let mut out = String::new();
        ser(&root, &mut out);
        out
    }

    pub fn deserialize(&self, data: String) -> Node {
        fn de<I: Iterator<Item=String>>(it: &mut I) -> Node {
            let v = it.next().unwrap();
            if v == "null" { return None; }
            let val: i32 = v.parse().unwrap();
            let node = Rc::new(RefCell::new(TreeNode { val, left: None, right: None }));
            node.borrow_mut().left = de(it);
            node.borrow_mut().right = de(it);
            Some(node)
        }
        let mut it = data.split(',').map(|x| x.to_string());
        de(&mut it)
    }
}
