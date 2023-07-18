use std::{fmt::Debug, fs, iter::Sum, ops::Add};

#[derive(Debug)]
struct BigInt(Vec<i64>);

impl From<&str> for BigInt {
    fn from(s: &str) -> BigInt {
        let t = s.trim();
        // println!("t.len(): {}", t.len());
        let mut parts: Vec<i64> = Vec::new();
        for i in 0..(t.len() / 10) {
            // println!("=================");
            let current = &t[(i * 10)..(i * 10 + 10)].trim();
            // println!("current: {}", current);
            // println!("current.len(): {}", current.len());
            // println!("diff: {}", (i * 10 + 10) - (i * 10));
            parts.push(current.parse().unwrap());
            // println!("{:?}", parts);
        }

        return BigInt(parts);
    }
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
        const MAX_PART_SIZE: i64 = 10000000000;

        println!("=================");
        let BigInt(lhs_vec) = self;
        let BigInt(rhs_vec) = rhs;

        let (longer, shorter) = match lhs_vec.len() > rhs_vec.len() {
            true => (lhs_vec, rhs_vec),
            false => (rhs_vec, lhs_vec),
        };

        let longer: Vec<&i64> = longer.iter().rev().collect();
        let shorter: Vec<&i64> = shorter.iter().rev().collect();
        println!("before shorter: {:?}", shorter);

        let mut carry: i64 = 0;
        let mut out_vec: Vec<i64> = Vec::new();
        for i in 0..longer.len() {
            let mut current: i64 = 0;
            match i < shorter.len() {
                true => {
                    println!("i: {}", i);
                    println!("longer[i]: {}", longer[i]);
                    println!("shorter[i]: {}", shorter[i]);
                    current = *longer[i] + *shorter[i] + carry;
                    match current >= MAX_PART_SIZE {
                        true => {
                            carry = current / MAX_PART_SIZE;
                            current = current - MAX_PART_SIZE;
                        }
                        false => {}
                    }
                }
                false => {
                    current = *longer[i] + carry;
                }
            }

            out_vec.push(current)
        }

        return BigInt(out_vec);
    }
}

impl Sum for BigInt {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut p = BigInt::from("0");
        for q in iter {
            p = p + q;
        }
        return p;
    }
}

impl BigInt {
    fn first_10(self) -> String {
        let BigInt(v) = self;
        let s = v[0].to_string();
        if s.len() < 10 {
            let l = 10 - s.len();
            let s2 = v[1].to_string();
            // let l2 = 10 - s2.len();
            let p2_padded = format!("{:0>10}", s2);
            return s + &p2_padded[0..l];
        } else {
            return s;
        }
    }
}

fn main() {
    let data = fs::read_to_string("data").unwrap();
    // let numbers: Vec<&str> = data.split("\n").collect::<Vec<&str>>()[0..3].to_vec();
    let numbers: Vec<&str> = data.split("\n").collect();
    let bigints: Vec<BigInt> = numbers.iter().map(|x| BigInt::from(*x)).collect();
    let result: BigInt = bigints.into_iter().sum();
    println!("result bigint: {:?}", result);
    println!("first_10: {}", result.first_10());
}
