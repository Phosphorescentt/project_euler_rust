fn main() {
    for a in 1..999 {
        for b in 1..a {
            let c = 1000 - a - b;
            if a < c && b < c {
                if (a as i32).pow(2) + (b as i32).pow(2) == (c as i32).pow(2) {
                    println!("{}", a * b * c);
                }
            }
        }
    }
}
