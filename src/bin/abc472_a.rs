use std::{io::Read};
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let s = input.trim();
    let ans: String = s.chars().map(|x| if x == 'A' { 'A' } else { '.' }).collect();
    println!("{ans}");
}