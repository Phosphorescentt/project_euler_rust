fn main() {
    let mut total = 0;
    for i in 0..1000 {
        if i % 3 == 0 {
            total += i;
        } else if i % 5 == 0 {
            total += i;
        } else {
            total += 0;
        }
    }

    println!("{}", total);
}
