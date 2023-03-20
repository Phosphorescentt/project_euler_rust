fn is_palindrome(x: i64) -> bool {
    let s = x.to_string();

    if s.chars().collect::<String>() == s.chars().rev().collect::<String>() {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut palindromes: Vec<i64> = Vec::new();
    for i in (0..1000).rev() {
        for j in (0..1000).rev() {
            println!("{}, {}", i, j);
            let n = i * j;
            if is_palindrome(n) {
                println!("{} is palindromic", n);
                palindromes.push(n);
                break;
            }
        }
    }

    println!("{:?}", palindromes);
    println!("{:?}", palindromes.iter().max());
}
