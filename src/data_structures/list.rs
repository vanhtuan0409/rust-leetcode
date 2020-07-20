#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
}

impl<T: Copy> ListNode<T> {
    #[allow(dead_code)]
    pub fn from(l: Vec<T>) -> Option<Box<Self>> {
        let mut ret = None;
        for &item in l.iter().rev() {
            let mut node = ListNode::new(item);
            node.next = ret;
            ret = Some(Box::new(node));
        }
        ret
    }
}
