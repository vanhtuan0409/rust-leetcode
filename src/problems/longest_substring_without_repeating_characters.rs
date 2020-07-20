#[allow(dead_code)]
pub struct Solution {}

// Leetcode link: https://leetcode.com/problems/longest-substring-without-repeating-characters/
impl Solution {
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".into()));
        assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".into()));
    }
}
