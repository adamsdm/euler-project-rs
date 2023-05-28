/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
*/

// Estimates the number of primes below a given limit.
fn estimate_number_of_primes(limit: u64) -> u64 {
    let limit_ln = (limit as f64).ln();
    let limit_ln = limit_ln.round() as u64;
    limit / limit_ln
}

// Estimates the range of values to generate primes for a given number of primes.
fn estimate_range(num_primes: u64) -> u64 {
    let mut limit = num_primes * 2;
    while estimate_number_of_primes(limit) < num_primes {
        limit = limit * 2;
    }
    limit
}

fn main() {
    let num_primes = 10_001;

    // Estimate how large the range will be,
    let max_in_range = estimate_range(num_primes);

    // Calculate primes up to max_in_range * 2, to ensure our n:th prime is in the range
    println!("Generating {max_in_range} primes...");
    let primes = primes::sieve_of_eratosthenes(max_in_range);

    println!(
        "The {num_primes}:th prime is {}, ({})",
        primes[num_primes as usize],
        primes.len()
    );
}
