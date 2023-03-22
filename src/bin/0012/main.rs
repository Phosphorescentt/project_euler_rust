#[derive(Default)]
struct TriangleNumbers {
    i: i64,
    current_number: i64,
}

impl Iterator for TriangleNumbers {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        self.current_number += self.i;
        return Some(self.current_number);
    }
}

fn no_of_divisors(x: i64) -> i64 {
    let max = (x as f64).sqrt() + 1.0;

    let mut count = 0;
    for i in 1..(max as i64) {
        if x % i == 0 {
            count += 2
        }
    }
    return count;
}

fn main() {
    let mut no_divisors = 0;
    let mut t = TriangleNumbers::default();
    while no_divisors <= 500 {
        let current = t.next().unwrap();
        no_divisors = no_of_divisors(current);
        println!("{}:{}", current, no_divisors);
    }

    println!("{}", t.current_number);
}
