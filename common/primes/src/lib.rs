pub fn sieve_of_eratosthenes(max: u64) -> Vec<u64> {
    // Generate vector
    let mut is_prime_vector = vec![true; (max + 1) as usize];

    // 0 and 1 is a special case and not a prime
    is_prime_vector[0] = false;
    is_prime_vector[1] = false;

    for number in 2..is_prime_vector.len() {
        if is_prime_vector[number] {
            // Remove all multiples of i if it is a prime, starting at i*2 to not remove i itself
            for j in (number * number..is_prime_vector.len()).step_by(number) {
                is_prime_vector[j] = false;
            }
        }
    }

    // Collect prime indices into vector and return
    return is_prime_vector
        .iter()
        .enumerate()
        .filter_map(
            |(index, &is_prime)| {
                if is_prime {
                    Some(index as u64)
                } else {
                    None
                }
            },
        )
        .collect();
}

#[test]
fn test_sieve() {
    assert_eq!(sieve_of_eratosthenes(20), [1, 2, 3, 5, 7, 11, 13, 17, 19]);
    assert_eq!(sieve_of_eratosthenes(19), [1, 2, 3, 5, 7, 11, 13, 17, 19]);
}
