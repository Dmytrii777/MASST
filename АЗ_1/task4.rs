fn climb_stairs(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        _ => climb_stairs(n - 1) + climb_stairs(n - 2),
    }
}

// Приклади:
// climb_stairs(2) == 2
// climb_stairs(3) == 3
