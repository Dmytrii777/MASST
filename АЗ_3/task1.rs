pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: None });
    let mut tail = &mut dummy;

    while list1.is_some() && list2.is_some() {
        let take_from_list1 =
            list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val;
        if take_from_list1 {
            let next = list1.as_mut().unwrap().next.take();
            tail.next = list1;
            tail = tail.next.as_mut().unwrap();
            list1 = next;
        } else {
            let next = list2.as_mut().unwrap().next.take();
            tail.next = list2;
            tail = tail.next.as_mut().unwrap();
            list2 = next;
        }
    }

    tail.next = if list1.is_some() { list1 } else { list2 };

    dummy.next
}
