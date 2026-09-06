use std::{io::Read, fmt::Write};
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();
    let k: usize = tokens.next().unwrap().parse().unwrap();
    let mut prev = 0;
    let mut a: Vec<usize> = vec![0; n];
    let mut ans: String = String::new();
    for i in 0..n {
        if i >= m {
            prev -= a[i - m];
        }
        let snack: usize = tokens.next().unwrap().parse().unwrap();
        if snack + prev <= k {
            a[i] = snack;
            prev += snack;
            write!(ans, "Yes\n").unwrap();
        } else {
            write!(ans, "No\n").unwrap();
        }
    }
    print!("{ans}");
}
