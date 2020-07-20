#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[allow(dead_code)]
    pub fn from(l: Vec<i32>) -> Option<Box<Self>> {
        let mut ret = None;
        for &item in l.iter().rev() {
            let mut node = ListNode::new(item);
            node.next = ret;
            ret = Some(Box::new(node));
        }
        ret
    }
}
