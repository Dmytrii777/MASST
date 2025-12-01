fn max_consecutive_ones(nums: &[i32]) -> i32 {
    let mut max_len = 0;
    let mut cur = 0;

    for &v in nums {
        if v == 1 {
            cur += 1;
            if cur > max_len {
                max_len = cur;
            }
        } else {
            cur = 0;
        }
    }

    max_len
}
