fn main() {
    println!("{}", lattice_paths(20));
}
fn lattice_paths(n: u128) -> u128 {
    return (0..=n).map(|k| n_choose_k(n, k) * n_choose_k(n, k)).sum();
}

fn n_choose_k(n: u128, k: u128) -> u128 {
    let numerator: u128 = (1..=n).product();
    let denominator: u128 = (1..=k).product::<u128>() * (1..=(n - k)).product::<u128>();
    return numerator / denominator;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_choose_k() {
        assert_eq!(n_choose_k(2, 0), 1);
        assert_eq!(n_choose_k(2, 1), 2);
        assert_eq!(n_choose_k(2, 2), 1);

        assert_eq!(n_choose_k(3, 0), 1);
        assert_eq!(n_choose_k(3, 1), 3);
        assert_eq!(n_choose_k(3, 2), 3);
        assert_eq!(n_choose_k(3, 3), 1);
    }

    #[test]
    fn test_2_1() {
        assert_eq!(lattice_paths(1), 2);
    }

    #[test]
    fn test_2_2() {
        assert_eq!(lattice_paths(2), 6);
    }
}
