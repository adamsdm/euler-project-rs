/*
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a2 + b2 = c2
For example, 32 + 42 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
 */

fn is_pythagorean_triplet(a: i64, b: i64, c: i64) -> bool {
    (a < b && b < c) && (a.pow(2) + b.pow(2) == c.pow(2))
}

// Finds the pythogorean triplet that has the given sum
// Returns an Option containing the tuple, or None if no tuple with the given sum was found
fn find_pythogorean_triplet_with_sum(sum: i64) -> Option<(i64, i64, i64)> {
    const LIMIT: i64 = 1000;
    for a in 1..LIMIT {
        for b in a..LIMIT {
            for c in b..LIMIT - a - b {
                if is_pythagorean_triplet(a, b, c) && a + c + b == sum {
                    let tuple = (a, b, c);
                    return Some(tuple);
                }
            }
        }
    }
    None
}

fn main() {
    const TUPLE_SUM: i64 = 1000;
    if let Some(tuple) = find_pythogorean_triplet_with_sum(TUPLE_SUM) {
        println!("Found tuple with the sum {TUPLE_SUM}: {:?}", tuple);
    } else {
        println!("Could not find a tuple with the sum {TUPLE_SUM}");
    }
}

#[test]
fn test_is_pythagorean_triplet() {
    assert_eq!(is_pythagorean_triplet(3, 4, 5), true);
    assert_eq!(is_pythagorean_triplet(4, 3, 5), false);
    assert_eq!(is_pythagorean_triplet(1, 1, 1), false);

    assert_eq!(find_pythogorean_triplet_with_sum(0), None);
}
