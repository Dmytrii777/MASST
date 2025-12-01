fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::with_capacity(nums.len());

    // спочатку парні
    for x in &nums {
        if x % 2 == 0 {
            res.push(*x);
        }
    }
    // потім непарні
    for x in &nums {
        if x % 2 != 0 {
            res.push(*x);
        }
    }

    res
}
