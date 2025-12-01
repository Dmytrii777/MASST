/// Видалити вузол, маючи лише посилання на нього (він не останній).
pub fn delete_node(node: &mut ListNode) {
    if let Some(next) = node.next.take() {
        node.val = next.val;
        node.next = next.next;
    }
}
