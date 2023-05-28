/* Problem 2 - Even Fibonacci numbers

Each new term in the Fibonacci sequence is generated by adding the previous two terms.
By starting with 1 and 2, the first 10 terms will be:

1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

By considering the terms in the Fibonacci sequence whose values do not exceed four million,
find the sum of the even-valued terms.
*/

fn fibonacci(max: u64) -> Vec<u64> {
    let mut res = vec![1,2];

    for n in 1..max-1 {
        let sum_of_last_two_elements = res.iter().rev().take(2).sum();
        res.push(sum_of_last_two_elements);
    }

    return res;
}


fn main() {
    println!("{:?}", fibonacci(10));
}

#[test]
fn test_fibonacci() {
    assert_eq!(fibonacci(10), [1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
}