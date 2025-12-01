pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;
    let k = k as usize;
    let mut dq = VecDeque::new();
    let mut ans = Vec::new();

    for i in 0..nums.len() {
        while let Some(&back) = dq.back() {
            if nums[back] < nums[i] {
                dq.pop_back();
            } else {
                break;
            }
        }

        dq.push_back(i);

        if dq.front().unwrap() + k <= i {
            dq.pop_front();
        }

        if i + 1 >= k {
            ans.push(nums[*dq.front().unwrap()]);
        }
    }

    ans
}
