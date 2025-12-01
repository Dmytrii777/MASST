fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
    if nums.is_empty() {
        return 0;
    }

    let mut k = 1usize; // позиція для наступного унікального елемента

    for i in 1..nums.len() {
        if nums[i] != nums[k - 1] {
            nums[k] = nums[i];
            k += 1;
        }
    }

    k
}
