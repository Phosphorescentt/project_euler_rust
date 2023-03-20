fn square_of_sum(n: i32) -> i32 {
    (1..(n + 1)).sum::<i32>().pow(2)
}

fn sum_of_square(n: i32) -> i32 {
    (1..(n + 1)).map(|x| x * x).sum()
}

fn main() {
    let n = 100;
    let diff = square_of_sum(n) - sum_of_square(n);
    println!("{}", diff);
}
