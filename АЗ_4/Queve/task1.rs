pub fn first_uniq_char(s: String) -> i32 {
    use std::collections::HashMap;

    let mut freq = HashMap::new();
    for ch in s.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }

    for (i, ch) in s.chars().enumerate() {
        if freq[&ch] == 1 {
            return i as i32;
        }
    }

    -1
}
