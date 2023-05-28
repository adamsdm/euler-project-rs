/* Problem 4 - Largest palindrome product

A palindromic number reads the same both ways.
The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
 */

fn is_palindrome(input: &str) -> bool {
    let mut input = input.to_string();

    // If an odd lenght of input..
    if input.len() % 2 != 0 {
        // Remove the middle char
        input.remove(input.len() / 2);
    }

    let (first, second) = input.split_at(input.len() / 2);
    return first == second.chars().rev().collect::<String>();
}

fn main() {
    let mut factor_a = 0;
    let mut factor_b = 0;
    let mut largest_product = 0;

    for i in 111..1000 {
        for j in 111..1000 {
            let product = i * j;
            let is_product_a_palindrome = is_palindrome(product.to_string().as_str());
            if product > largest_product && is_product_a_palindrome {
                largest_product = product;
                factor_a = i;
                factor_b = j;
            }
        }
    }

    println!("The largets palindrome is {largest_product}={factor_a}*{factor_b}");
}

#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome("9009"), true);
    assert_eq!(is_palindrome("lol"), true);
    assert_eq!(is_palindrome("123#321"), true);
}
