/// Подвоїти число, представлене списком цифр у прямому порядку.
pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut digits = collect_vals(&head);
    let mut carry = 0;

    for d in digits.iter_mut().rev() {
        let v = *d * 2 + carry;
        *d = v % 10;
        carry = v / 10;
    }
    if carry > 0 {
        digits.insert(0, carry);
    }

    from_vals(&digits)
}
