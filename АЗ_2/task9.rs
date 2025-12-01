fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
    let mut max_right = -1;

    for i in (0..arr.len()).rev() {
        let cur = arr[i];
        arr[i] = max_right;
        if cur > max_right {
            max_right = cur;
        }
    }

    arr
}
