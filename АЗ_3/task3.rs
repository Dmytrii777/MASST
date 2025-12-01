use std::ptr;

/// Floyd (slow/fast) для виявлення циклу
pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    let mut slow = head.as_deref();
    let mut fast = head.as_deref();

    while let (Some(s), Some(f)) = (slow, fast) {
        if let Some(next_fast) = f.next.as_deref() {
            slow = s.next.as_deref();
            fast = next_fast.next.as_deref();
        } else {
            break;
        }
        if let (Some(s1), Some(f1)) = (slow, fast) {
            if ptr::eq::<ListNode>(s1, f1) {
                return true;
            }
        }
    }

    false
}
