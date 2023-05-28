/*
* The sum of the squares of the first ten natural numbers is,
1^2 + 2^2 ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,
(1 + 2 ... + 10)^2 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is
3025 - 385 = 2640

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

fn sum_of_squares(max: u64) -> u64 {
    (1..=max).map(|x| x.pow(2)).sum()
}

fn square_of_sum(max: u64) -> u64 {
    (1..=max).sum::<u64>().pow(2)
}

fn main() {
    assert_eq!(sum_of_squares(10), 385);
    assert_eq!(square_of_sum(10), 3025);
}
