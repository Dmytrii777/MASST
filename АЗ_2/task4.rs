fn duplicate_zeros(arr: &mut [i32]) {
    let n = arr.len();
    let mut zeros = 0usize;

    for &v in arr.iter() {
        if v == 0 {
            zeros += 1;
        }
    }

    let mut i: isize = n as isize - 1;
    let mut j: isize = n as isize - 1 + zeros as isize;

    // заповнюємо з кінця, враховуючи "розширений" індекс j
    while i >= 0 && j >= 0 {
        if j < n as isize {
            arr[j as usize] = arr[i as usize];
        }
        if arr[i as usize] == 0 {
            j -= 1;
            if j < n as isize {
                arr[j as usize] = 0;
            }
        }
        i -= 1;
        j -= 1;
    }
}
