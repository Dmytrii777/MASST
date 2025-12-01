fn count_even_digit_numbers(nums: &[i32]) -> i32 {
    fn digit_count(mut x: i32) -> i32 {
        let mut cnt = 0;
        while x > 0 {
            cnt += 1;
            x /= 10;
        }
        if cnt == 0 { 1 } else { cnt }
    }

    let mut res = 0;
    for &n in nums {
        if digit_count(n) % 2 == 0 {
            res += 1;
        }
    }
    res
}
