fn is_prime(n: i32) -> bool {
    let max = (n as f32).sqrt();

    for i in 2..(max as i32 + 1) {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

fn main() {
    let mut count = 0;
    let mut i = 2;
    while count != 10_001 {
        if is_prime(i) {
            count += 1;
        } else {
            // do nothing
        }

        println!("{}", i);
        i += 1;
    }
}
