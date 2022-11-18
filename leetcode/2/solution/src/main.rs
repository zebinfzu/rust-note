#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
mod solution {
    use crate::ListNode;

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut tail = &mut result;
        let (mut l1, mut l2) = (l1, l2);
        let mut t = 0;
        // 枚举所有的状况
        loop {
            if let (None, None, 0) = (&l1, &l2, t) {
                break;
            }
            if let Some(node) = l1.to_owned() {
                t += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2.to_owned() {
                t += node.val;
                l2 = node.next;
            }
            *tail = Some(Box::new(ListNode::new(t % 10)));
            t /= 10;
            tail = &mut tail.as_mut().unwrap().next;
        }
        result
    }
}
fn main() {}
