pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::VecDeque;

    let k = k as usize;
    let mut dp = vec![0; nums.len()];
    let mut dq = VecDeque::new();
    let mut best = i32::MIN;

    for i in 0..nums.len() {
        if let Some(&front) = dq.front() {
            if front + k < i {
                dq.pop_front();
            }
        }

        dp[i] = nums[i];
        if let Some(&front) = dq.front() {
            dp[i] = dp[i].max(dp[front] + nums[i]);
        }

        while let Some(&back) = dq.back() {
            if dp[back] <= dp[i] {
                dq.pop_back();
            } else {
                break;
            }
        }

        dq.push_back(i);
        best = best.max(dp[i]);
    }

    best
}
