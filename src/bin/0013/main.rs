fn main() {
    let data: Vec<&str> = include_str!("data").lines().collect();
    let width = data.get(0).unwrap().len();
    let height = data.len();

    dbg!(width);
    dbg!(height);

    let mut total: u64 = 0;
    let mut exp: u32 = 0;
    for y in 0..width {
        dbg!(y);
        let mut column_total: u64 = 0;
        for x in 0..height {
            let add_num: u64 = data
                .get(x)
                .unwrap()
                .chars()
                .nth(width - 1 - y)
                .unwrap()
                .to_string()
                .parse::<u64>()
                .unwrap();

            column_total += add_num;
        }
        dbg!(column_total);
        total += column_total * u64::pow(10, exp);

        exp += 1;

        if total.to_string().len() > 10 {
            total = total / 10;
            exp -= 1;
        }
    }

    println!("num: {}", total);
}
