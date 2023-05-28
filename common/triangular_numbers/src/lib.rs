pub fn get_nth_triangular_number(n: usize) -> u64 {
    (n * (n + 1) / 2) as u64
}

pub fn gen_triangular_numbers(limit: usize) -> Vec<u64> {
    (1..=limit).map(|x| get_nth_triangular_number(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nth_triangular_number() {
        assert_eq!(get_nth_triangular_number(0), 0);
        assert_eq!(get_nth_triangular_number(1), 1);
        assert_eq!(get_nth_triangular_number(2), 3);
        assert_eq!(get_nth_triangular_number(3), 6);
        assert_eq!(get_nth_triangular_number(4), 10);
        assert_eq!(get_nth_triangular_number(5), 15);
    }

    #[test]
    fn test_gen_triangular_numbers() {
        assert_eq!(
            gen_triangular_numbers(10),
            vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55]
        );
    }
}
