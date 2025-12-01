fn valid_mountain_array(arr: &[i32]) -> bool {
    let n = arr.len();
    if n < 3 {
        return false;
    }

    let mut i = 0usize;

    // підйом
    while i + 1 < n && arr[i] < arr[i + 1] {
        i += 1;
    }

    // пік не може бути на краях
    if i == 0 || i == n - 1 {
        return false;
    }

    // спуск
    while i + 1 < n && arr[i] > arr[i + 1] {
        i += 1;
    }

    i == n - 1
}
