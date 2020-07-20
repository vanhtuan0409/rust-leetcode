use crate::data_structures::list;

type ListNode = list::ListNode<i32>;

#[allow(dead_code)]
pub struct Solution {}

// Leetcode link: https://leetcode.com/problems/add-two-numbers/
impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut it1, mut it2) = (l1, l2);
        let mut overflow = 0;
        let mut ret = None;
        println!("======");
        loop {
            let mut both_list_ended = 0;
            let lhs = match it1 {
                Some(item) => {
                    it1 = item.next;
                    item.val
                }
                None => {
                    both_list_ended |= 1;
                    0
                }
            };
            let rhs = match it2 {
                Some(item) => {
                    it2 = item.next;
                    item.val
                }
                None => {
                    both_list_ended |= 2;
                    0
                }
            };
            let result = lhs + rhs + overflow;
            overflow = (result) / 10;
            if both_list_ended == 3 && result == 0 {
                break;
            }
            let mut new_node = ListNode::new(result % 10);
            new_node.next = ret;
            ret = Some(Box::new(new_node));
        }
        Solution::reverse_list(ret)
    }

    #[allow(dead_code)]
    fn reverse_list(l: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ret = None;
        let mut it = l;
        loop {
            match it {
                Some(item) => {
                    let mut node = ListNode::new(item.val);
                    node.next = ret;
                    ret = Some(Box::new(node));
                    it = item.next;
                }
                None => break,
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(
            ListNode::from(vec![3, 2, 1]),
            super::Solution::reverse_list(ListNode::from(vec![1, 2, 3]))
        )
    }

    #[test]
    fn test() {
        assert_eq!(
            ListNode::from(vec![8, 0, 7]),
            Solution::add_two_numbers(ListNode::from(vec![3, 4, 2]), ListNode::from(vec![4, 6, 5]))
        );
        assert_eq!(
            ListNode::from(vec![2, 4, 3]),
            Solution::add_two_numbers(ListNode::from(vec![2, 4, 3]), None,)
        );
        assert_eq!(
            ListNode::from(vec![5, 6, 4]),
            Solution::add_two_numbers(None, ListNode::from(vec![5, 6, 4]))
        );
        assert_eq!(None, Solution::add_two_numbers(None, None,));

        assert_eq!(
            ListNode::from(vec![1, 0]),
            Solution::add_two_numbers(ListNode::from(vec![5]), ListNode::from(vec![5]))
        );
    }
}
