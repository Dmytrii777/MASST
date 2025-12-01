#[derive(Debug, PartialEq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() || head.as_ref().unwrap().next.is_none() {
        return head;
    }
    let mut first = head.unwrap();
    let mut second = first.next.unwrap();
    let next_next = second.next.take();
    first.next = swap_pairs(next_next);
    second.next = Some(first);
    Some(second)
}
