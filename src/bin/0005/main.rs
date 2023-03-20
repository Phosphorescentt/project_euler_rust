// There is a much more efficient solution involving looking at the prime
// factorisation of 20! vs the union of the prime factors of 1..21. Finding the intersection
// of those two and then applying the product should yield the result. I'm
// just too tiredd to do that right now :)

fn main() {
    let max = 20;
    let mut i = 0;
    let mut remainders: Vec<i32> = vec![1; max];
    while remainders != vec![0; max] {
        i += 1;
        remainders = (1..(max + 1)).map(|x| i % x as i32).collect::<Vec<i32>>();
        println!("{}", i);
    }
}
