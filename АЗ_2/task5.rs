fn merge(nums1: &mut Vec<i32>, m: usize, nums2: &[i32], n: usize) {
    let mut i: isize = m as isize - 1;
    let mut j: isize = n as isize - 1;
    let mut k: isize = (m + n) as isize - 1;

    while j >= 0 {
        if i >= 0 && nums1[i as usize] > nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        k -= 1;
    }
}
