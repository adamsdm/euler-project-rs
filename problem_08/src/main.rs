/*
The four adjacent digits in the 1000-digit number that have the
greatest product are 9 × 9 × 8 × 9 = 5832.


Find the thirteen adjacent digits in the 1000-digit number that have the greatest product.
What is the value of this product?
*/

use std::fs;

fn read_number() -> String {
    fs::read_to_string("number.txt")
        .expect("Should have been able to read the file")
        .replace("\n", "")
}

fn product_of_digits(s: &str) -> u64 {
    let mut product = 1;

    for c in s.chars() {
        if let Some(digit) = c.to_digit(10) {
            product *= digit as u64;
        }
    }

    product
}

fn find_largest_product(number: &str, sieve_size: usize) -> u64 {
    let mut largets_product = 0;
    for sieve_start in 0..=number.len() - sieve_size {
        let sieve = &number[sieve_start..sieve_start + sieve_size];
        let product = product_of_digits(sieve);
        if product > largets_product {
            largets_product = product;
        }
    }
    largets_product
}

fn main() {
    const SIEVE_SIZE: usize = 4;

    let number = read_number();
    let largest_product = find_largest_product(&number, SIEVE_SIZE);
    println!("The largest product of {SIEVE_SIZE} consequtive numbers are {largest_product}");
}
