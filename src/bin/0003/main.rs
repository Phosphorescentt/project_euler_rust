fn is_prime(x: i64) -> bool {
    let max = (x as f64).sqrt() + 1.0;
    for i in 2..(max as i64) {
        if x % i == 0 {
            return false;
        }
    }

    return true;
}

fn largest_prime_factor(x: i64) -> i64 {
    let max = ((x as f64) / 2.0) + 1.0;
    for i in 2..(max as i64) {
        if x % i == 0 {
            println!("Checking primality: {}", x / i);
            if is_prime(x / i) {
                return x / i;
            }
        }
    }

    return -1;
}

fn main() {
    let x = largest_prime_factor(600851475143);
    println!("{}", x);
}
