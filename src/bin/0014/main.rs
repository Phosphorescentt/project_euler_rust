use std::collections::HashMap;

fn main() {
    let mut found = setup_found();

    let mut n_max: u64 = 4;
    for i in 1..=1_000_000 {
        if let Some(len) = collatz_recurse(i, &mut found) {
            if len > found.get(&n_max).unwrap().unwrap() {
                n_max = i;
                println!("n_max: {}, len: {}", n_max, len);
            }
        }
    }

    // println!("found: {:?}", found);
    println!("n_max: {}", n_max);
    println!("len: {}", found.get(&n_max).unwrap().unwrap());
}

fn setup_found() -> HashMap<u64, Option<u64>> {
    let mut found: HashMap<u64, Option<u64>> = HashMap::new();
    found.insert(1, Some(1));
    found.insert(2, Some(2));
    found.insert(4, Some(3));

    return found;
}

fn collatz_recurse(n: u64, found: &mut HashMap<u64, Option<u64>>) -> Option<u64> {
    if let Some(chain_length_option) = found.get(&n) {
        return *chain_length_option;
    } else {
        if let Some(next_number) = collatz_calculate(n) {
            if let Some(sub_length) = collatz_recurse(next_number, found) {
                found.insert(n, Some(sub_length + 1));
                return Some(sub_length + 1);
            } else {
                found.insert(n, None);
                return None;
            }
        } else {
            return None;
        }
    }
}

fn collatz_calculate(n: u64) -> Option<u64> {
    if (n % 2) == 0 {
        return Some(n / 2);
    } else {
        return Some(3 * n + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_eq_13() {
        let mut found = setup_found();
        assert_eq!(collatz_recurse(13, &mut found), Some(10));
    }

    #[test]
    fn n_eq_9() {
        let mut found = setup_found();
        assert_eq!(collatz_recurse(9, &mut found), Some(20));
    }

    #[test]
    fn n_eq_999999() {
        let mut found = setup_found();
        assert_eq!(collatz_recurse(999999, &mut found), None);
    }

    #[test]
    fn n_eq_595800() {
        let mut found = setup_found();
        assert_eq!(collatz_recurse(595800, &mut found), Some(297));
    }

    #[test]
    fn n_eq_4() {
        let mut found = setup_found();
        assert_eq!(collatz_recurse(4, &mut found), Some(3));
    }

    #[test]
    fn n_eq_10971() {
        let mut found = setup_found();
        assert_eq!(collatz_recurse(10971, &mut found), Some(268));
    }

    #[test]
    fn multiple_test() {
        let mut found = setup_found();
        assert_eq!(collatz_recurse(9, &mut found), Some(20));
        assert_eq!(collatz_recurse(595800, &mut found), Some(297));
    }
}
