use std::{io::Read, fmt::Write};
const P: usize = 998244353;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
}

fn exp(a: usize, b: usize) -> usize {
    let mut ans = 1;
    let mut x = a % P;
    let mut y = b % (P - 1);
    while y != 0 {
        if y & 1 == 1 {
            ans = ans * x % P;
        }
        x = x * x % P;
        y >>= 1;
    }
    ans
}

fn inv(a: usize) -> usize {
    exp(a, P - 2)
}