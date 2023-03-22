fn is_prime(n: i64) -> bool {
    let max = (n as f64).sqrt();

    for i in 2..(max as i64 + 1) {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

fn main() {
    let mut total = 0;

    for i in 2..2_000_001 {
        if is_prime(i) {
            total += i;
        }
    }

    println!("{}", total);
}
