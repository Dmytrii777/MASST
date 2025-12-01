pub fn recover_from_preorder(s: String) -> Node {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    fn parse(chars: &Vec<char>, i: &mut usize, depth: usize) -> Node {
        let mut d = 0;
        let n = chars.len();

        let start = *i;
        while *i < n && chars[*i] == '-' {
            d += 1;
            *i += 1;
        }
        if d != depth { 
            *i = start;
            return None; 
        }

        let mut val = 0;
        while *i < n && chars[*i].is_numeric() {
            val = val * 10 + (chars[*i] as i32 - '0' as i32);
            *i += 1;
        }

        let node = Rc::new(RefCell::new(TreeNode { val, left: None, right: None }));
        node.borrow_mut().left = parse(chars, i, depth+1);
        node.borrow_mut().right = parse(chars, i, depth+1);

        Some(node)
    }

    parse(&chars, &mut i, 0)
}
