fn sorted_squares(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut res = vec![0; n];

    let mut left: isize = 0;
    let mut right: isize = n as isize - 1;
    let mut pos: isize = n as isize - 1;

    while left <= right {
        let l_val = nums[left as usize];
        let r_val = nums[right as usize];
        let l_sq = l_val * l_val;
        let r_sq = r_val * r_val;

        if l_sq > r_sq {
            res[pos as usize] = l_sq;
            left += 1;
        } else {
            res[pos as usize] = r_sq;
            right -= 1;
        }
        pos -= 1;
    }

    res
}
