#[allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut memoized = HashMap::new();
        for (i, item) in nums.iter().enumerate() {
            match memoized.get(&(target - item)) {
                Some(index) => return vec![*index, i as i32],
                None => {
                    memoized.insert(item, i as i32);
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}
