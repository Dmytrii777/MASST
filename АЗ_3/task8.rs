/// Реверсування вузлів у групах по k (через значення).
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let k = k as usize;
    let vals = collect_vals(&head);
    let n = vals.len();
    if k <= 1 || n == 0 {
        return head;
    }

    let mut res = Vec::with_capacity(n);
    let mut i = 0usize;

    while i + k <= n {
        for j in (i..i + k).rev() {
            res.push(vals[j]);
        }
        i += k;
    }
    while i < n {
        res.push(vals[i]);
        i += 1;
    }

    from_vals(&res)
}
