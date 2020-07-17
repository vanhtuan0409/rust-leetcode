mod problems;

use problems::two_sum::Solution;

fn main() {
    println!("{:?}", Solution::two_sum(vec![1, 2, 3], 6));
}
