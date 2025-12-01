fn print_reverse(s: &str) {
    if !s.is_empty() {
        print_reverse(&s[1..]);
        print!("{}", s.as_bytes()[0] as char);
    }
}

// Приклад використання:
// print_reverse("tiger"); // виводить "regit"
