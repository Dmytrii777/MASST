/// Об’єднати k відсортованих списків через збирання всіх значень і сортування.
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut vals = Vec::<i32>::new();

    for mut head in lists {
        let mut cur = head.take();
        while let Some(node) = cur {
            vals.push(node.val);
            cur = node.next;
        }
    }

    if vals.is_empty() {
        return None;
    }

    vals.sort();
    from_vals(&vals)
}
