use std::collections::HashMap;

fn main() {
    let mut found = setup_found();

    let mut n_max: u64 = 4;
    for i in 1..=1_000_000 {
        let len = collatz_recurse(i, &mut found);
        if len > *found.get(&n_max).unwrap() {
            n_max = i;
            println!("n_max: {}, len: {}", n_max, len);
        }
    }

    println!("n_max: {}", n_max);
    println!("len: {}", found.get(&n_max).unwrap());
}

fn setup_found() -> HashMap<u64, u64> {
    let mut found: HashMap<u64, u64> = HashMap::new();
    found.insert(1, 1);
    found.insert(2, 2);
    found.insert(4, 3);

    return found;
}

fn collatz_recurse(n: u64, found: &mut HashMap<u64, u64>) -> u64 {
    let l = {
        if let Some(chain_length) = found.get(&n) {
            *chain_length
        } else {
            let next_number = collatz_calculate(n);
            let sub_length = collatz_recurse(next_number, found);
            sub_length + 1
        }
    };

    found.insert(n, l);
    return l;
}

fn collatz_calculate(n: u64) -> u64 {
    if (n % 2) == 0 {
        return n / 2;
    } else {
        return 3 * n + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_eq_13() {
        let mut found = setup_found();
        assert_eq!(collatz_recurse(13, &mut found), 10);
    }

    #[test]
    fn n_eq_9() {
        let mut found = setup_found();
        assert_eq!(collatz_recurse(9, &mut found), 20);
    }

    #[test]
    fn n_eq_595800() {
        let mut found = setup_found();
        assert_eq!(collatz_recurse(595800, &mut found), 297);
    }

    #[test]
    fn n_eq_4() {
        let mut found = setup_found();
        assert_eq!(collatz_recurse(4, &mut found), 3);
    }

    #[test]
    fn n_eq_10971() {
        let mut found = setup_found();
        assert_eq!(collatz_recurse(10971, &mut found), 268);
    }

    #[test]
    fn multiple_test() {
        let mut found = setup_found();
        assert_eq!(collatz_recurse(9, &mut found), 20);
        assert_eq!(collatz_recurse(595800, &mut found), 297);
    }
}
