use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let _ = lines.next().unwrap();
    let mut x: u32 = 10000;
    let mut y: u32 = 10000;
    while let Some(line) = lines.next() {
        let mut abs = line.split_whitespace();
        let a: u32 = abs.next().unwrap().parse().unwrap();
        let b: u32 = abs.next().unwrap().parse().unwrap();
        let s = abs.next().unwrap();
        if s == "keep" {
            x = x.saturating_sub(b);
        } else {
            x = x.saturating_sub(a);
        }
        y = y.saturating_sub(a);
    }
    println!("{}", y - x);
}