/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
 */

fn main() {
    const TWO_MILLION: u64 = 2_000_000;
    println!(
        "The sum of all primes below {TWO_MILLION} is {}",
        primes::sieve_of_eratosthenes(TWO_MILLION)
            .iter()
            .sum::<u64>()
    );
}

#[test]
fn test_sum_primes() {
    assert_eq!(primes::sieve_of_eratosthenes(10).iter().sum::<u64>(), 17);
}
