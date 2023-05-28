/* Problem 5 - Smallest multiple
2520 is the smallest number that can be divided by
each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly
divisible by all of the numbers from 1 to 20?
*/

fn is_divisible_by_all(num: u64, r: std::ops::Range<u64>) -> bool {
    r.into_iter().all(|i| num % i == 0)
}

fn find_smallest_divisible(max: u64) -> u64 {
    let mut num = max;
    loop {
        if is_divisible_by_all(num, 1..max) {
            return num;
        }
        num += max;
    }
}

fn main() {
    println!(
        "The smallest number that is divisible by all numbers in the range 1..20 is {}",
        find_smallest_divisible(20)
    );
}

#[test]
fn test_is_divisible_by_all() {
    assert_eq!(2520, find_smallest_divisible(10));
}
