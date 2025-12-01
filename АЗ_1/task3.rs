fn fibonacci(n: usize) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Приклади:
// fibonacci(2) == 1
// fibonacci(3) == 2
// fibonacci(4) == 3
