pub fn decode_string(s: String) -> String {
    let mut num_stack = Vec::new();
    let mut str_stack = Vec::new();
    let mut curr = String::new();
    let mut num = 0;

    for ch in s.chars() {
        if ch.is_digit(10) {
            num = num * 10 + ch.to_digit(10).unwrap() as usize;
        } else if ch == '[' {
            num_stack.push(num);
            str_stack.push(curr.clone());
            curr.clear();
            num = 0;
        } else if ch == ']' {
            let repeat = num_stack.pop().unwrap();
            let mut tmp = str_stack.pop().unwrap();
            tmp.push_str(&curr.repeat(repeat));
            curr = tmp;
        } else {
            curr.push(ch);
        }
    }

    curr
}
