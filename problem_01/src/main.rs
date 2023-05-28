/*
Problem 01 - Multiples of 3 or 5

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
 */

fn sum_multiples(max: u64) -> u64 {
    (1..max).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

fn main() {
    let max = 1000;
    println!("sum_multiples({}) = {}", max, sum_multiples(1000));
}

#[test]
fn test_sum_multiples() {
    assert_eq!(sum_multiples(10), 23);
}
