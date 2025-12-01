pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head.as_mut();

    while let Some(node) = cur {
        while let Some(next) = node.next.as_ref() {
            if next.val == node.val {
                // пропускаємо дублікат
                let next_next = node.next.as_mut().unwrap().next.take();
                node.next = next_next;
            } else {
                break;
            }
        }
        cur = node.next.as_mut();
    }

    head
}
