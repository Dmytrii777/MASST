fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        1.0
    } else if n < 0 {
        1.0 / my_pow(x, -n)
    } else {
        let y = my_pow(x, n / 2);
        if n % 2 == 0 {
            y * y
        } else {
            y * y * x
        }
    }
}

// Приклади:
// my_pow(2.0, 10) == 1024.0
// my_pow(2.1, 3) ≈ 9.261
// my_pow(2.0, -2) == 0.25
