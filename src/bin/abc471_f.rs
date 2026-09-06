use std::{cmp::Reverse, io::Read};
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let k: usize = tokens.next().unwrap().parse().unwrap();
    let mut s: Vec<String> = tokens.map(|c| c.to_string()).collect();
    s.sort_by_key(|c| Reverse((c.len(), c.parse::<usize>().unwrap())));
    let mut cand1: Vec<String> = s.clone().into_iter().take(k).collect();
    let mut cand2: Vec<String> = s.clone().into_iter().take(k-1).collect();
    let mut m = "0".to_string();
    for i in (k-1)..n {
        if s[i].parse::<usize>().unwrap() > m.parse::<usize>().unwrap() || s[i].parse::<usize>().unwrap() == m.parse::<usize>().unwrap() && s[i].len() >= m.len() {
            m = s[i].clone();
        }
    }
    cand2.push(m);
    cand1.sort_by(|a, b| (a.to_string() + b).cmp(&(b.to_string() + a)));
    cand1.reverse();
    cand2.sort_by(|a, b| (a.to_string() + b).cmp(&(b.to_string() + a)));
    cand2.reverse();
    let ans1: String = cand1.join("").chars().skip_while(|&c| c == '0').collect();
    let ans2: String = cand2.join("").chars().skip_while(|&c| c == '0').collect();
    if ans1.len() > ans2.len() || ans1.len() == ans2.len() && ans1 >= ans2 {
        println!("{}", if ans1.is_empty() {"0".to_string()} else {ans1});
    } else {
        println!("{}", if ans2.is_empty() {"0".to_string()} else {ans2});
    }
}
