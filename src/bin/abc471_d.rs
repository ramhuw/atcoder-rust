use std::{fmt::Write, collections::BinaryHeap, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let q: usize = tokens.next().unwrap().parse().unwrap();
    let v: i64 = tokens.next().unwrap().parse().unwrap();
    let mut batteries: BinaryHeap<i64> = BinaryHeap::new();
    let mut ans = String::new();
    for _ in 0..q {
        if tokens.next().unwrap() == "1" {
            let tq: i64 = tokens.next().unwrap().parse().unwrap();
            let vq: i64 = tokens.next().unwrap().parse().unwrap();
            batteries.push(vq - tq);
        } else {
            let tq: i64 = tokens.next().unwrap().parse().unwrap();
            if let Some(b) = batteries.pop() {
                write!(ans, "{}\n", v.min(b + tq)).unwrap();
            } else {
                write!(ans, "-1\n").unwrap();
            }
        }
    }
    println!("{ans}");
}