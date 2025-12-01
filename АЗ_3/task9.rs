/// Розділити список на < x та >= x, зберігаючи відносний порядок.
pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut less_dummy = Box::new(ListNode { val: 0, next: None });
    let mut ge_dummy   = Box::new(ListNode { val: 0, next: None });

    let mut less_tail = &mut less_dummy;
    let mut ge_tail   = &mut ge_dummy;

    let mut cur = head;

    while let Some(mut node) = cur {
        cur = node.next.take();
        if node.val < x {
            less_tail.next = Some(node);
            less_tail = less_tail.next.as_mut().unwrap();
        } else {
            ge_tail.next = Some(node);
            ge_tail = ge_tail.next.as_mut().unwrap();
        }
    }

    less_tail.next = ge_dummy.next;
    less_dummy.next
}
