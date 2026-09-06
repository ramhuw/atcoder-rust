use std::{collections::HashMap, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let mut map: HashMap<String, usize> = HashMap::new();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let mut k: usize = 0;
    for _ in 0..n {
        let s = tokens.next().unwrap().to_lowercase();
        let e = map.entry(s).or_insert(0);
        *e += 1;
        if *e > k {
            k = *e;
        }
    }
    println!("{k}");
}