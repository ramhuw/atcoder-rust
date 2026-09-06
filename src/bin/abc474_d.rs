use std::{fmt::Write, io::Read};
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let mut a = vec![];
    for _ in 0..n {
        a.push(tokens.next().unwrap());
    }
    let mut b = vec![];
    for _ in 0..n {
        b.push(tokens.next().unwrap());
    }
    let mut m: usize = 0;
    let mut found = false;
    for i in 0..n {
        if a[i] > b[i] {
            m = i;
            found = true;
            break;
        }
    }
    let mut ans = String::new();
    if found {
        write!(ans, "Yes\n").unwrap();
        let mut i = 0;
        while i < n {
            if i == m {
                write!(ans, "1000000000000000000 ").unwrap();
            } else {
                write!(ans, "{} ", 1).unwrap();
            }
            i += 1;
        }
    } else {
        write!(ans, "No").unwrap();
    }
    println!("{ans}");
}
