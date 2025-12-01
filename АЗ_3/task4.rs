/// Перевпорядкування L0 → Ln → L1 → Ln-1 → ...
/// Реалізація через перестановку значень (для простоти).
pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let vals = collect_vals(head);
    let n = vals.len();
    if n <= 2 {
        return;
    }

    let mut res = Vec::with_capacity(n);
    let mut i = 0usize;
    let mut j = n - 1;

    while i <= j {
        if i == j {
            res.push(vals[i]);
        } else {
            res.push(vals[i]);
            res.push(vals[j]);
        }
        i += 1;
        if j == 0 { break; }
        j -= 1;
    }

    *head = from_vals(&res);
}
