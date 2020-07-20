#[allow(dead_code)]
pub struct Solution {}

use std::cmp::max;
use std::collections::HashSet;

// Leetcode link: https://leetcode.com/problems/longest-substring-without-repeating-characters/
impl Solution {
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_length: i32 = 0;
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() {
            let mut visited = HashSet::new();
            for j in i..chars.len() {
                if visited.contains(&chars[j]) {
                    break;
                } else {
                    max_length = max(max_length, (j - i + 1) as i32);
                    visited.insert(chars[j]);
                }
            }
        }
        max_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".into()));
        assert_eq!(1, Solution::length_of_longest_substring("bbbbb".into()));
        assert_eq!(3, Solution::length_of_longest_substring("pwwkew".into()));
    }
}
