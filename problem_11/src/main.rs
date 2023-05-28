use grid::*;
use std::fs;

fn read_number() -> String {
    fs::read_to_string("src/number.txt").expect("Should have been able to read the file")
}

fn build_grid() -> [[u64; 20]; 20] {
    let num = read_number();
    let mut grid = [[0u64; 20]; 20]; // Initialize a 20x20 array of u64s with zeros

    for (i, line) in num.lines().enumerate() {
        for (j, number) in line.split_whitespace().enumerate() {
            grid[i][j] = number.parse().unwrap();
        }
    }

    grid
}

fn main() {
    const NUM_ADJECENT_NUMBERS: usize = 4;

    let grid = build_grid();

    let mut largest_product = 0;

    for row in 0..=20 - NUM_ADJECENT_NUMBERS {
        for col in 0..=20 - NUM_ADJECENT_NUMBERS {
            let horizontal_slize: Vec<u64> = vec![
                grid[row][col],
                grid[row][col + 1],
                grid[row][col + 2],
                grid[row][col + 3],
            ];
            let vertical_slize = vec![
                grid[row][col],
                grid[row + 1][col],
                grid[row + 2][col],
                grid[row + 3][col],
            ];

            let diagonal_slize = vec![
                grid[row][col],
                grid[row + 1][col + 1],
                grid[row + 2][col + 2],
                grid[row + 3][col + 3],
            ];

            largest_product = largest_product.max(horizontal_slize.iter().product());
            largest_product = largest_product.max(vertical_slize.iter().product());
            largest_product = largest_product.max(diagonal_slize.iter().product());
        }
    }

    println!("The largest product is {largest_product}");
}
