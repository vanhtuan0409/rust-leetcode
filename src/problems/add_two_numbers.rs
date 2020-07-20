use crate::data_structures::list::ListNode;

#[allow(dead_code)]
pub struct Solution {}

// Leetcode link: https://leetcode.com/problems/add-two-numbers/
impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(l), None) => return Some(l),
            (None, Some(l)) => return Some(l),
            (None, None) => return None,
            _ => {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            ListNode::from(vec![7, 0, 8]),
            super::Solution::add_two_numbers(
                ListNode::from(vec![2, 4, 3]),
                ListNode::from(vec![5, 6, 4])
            )
        )
    }
}
