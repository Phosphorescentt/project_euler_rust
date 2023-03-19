#[derive(Debug)]
struct Fib {
    i: i64,
    x: i64,
    y: i64,
}

impl Default for Fib {
    fn default() -> Self {
        return Self { i: 0, x: 1, y: 0 };
    }
}

impl Iterator for Fib {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.i += 1;
            return Some(0);
        } else if self.i == 1 {
            self.i += 1;
            return Some(1);
        }
        let next = self.x + self.y;
        self.y = self.x;
        self.x = next;

        return Some(next);
    }
}

fn main() {
    let mut f = Fib::default();
    let mut i = 0;
    let mut total = 0;
    let mut x = f.next();

    while x.unwrap() < 4_000_000 {
        // while x.unwrap() < 100 {
        match x {
            Some(x) => {
                if x % 2 == 0 {
                    total += x;
                }
            }
            None => {}
        }

        x = f.next();
        i += 1;
    }

    println!("{}", total);
}
