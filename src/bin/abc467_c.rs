use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut nm = lines.next().unwrap().split_whitespace().map(|a| a.parse::<i128>().unwrap());
    let n = nm.next().unwrap() as usize;
    let m = nm.next().unwrap();
    let aa: Vec<i128> = lines.next().unwrap().split_whitespace().map(|a| a.parse::<i128>().unwrap()).collect();
    let bb: Vec<i128> = lines.next().unwrap().split_whitespace().map(|a| a.parse::<i128>().unwrap()).collect();
    let mut cc: Vec<i128> = vec![];
    for i in 0..(n-1) {
        cc.push((bb[i] + 2 * m - aa[i] - aa[i+1]) % m);
    }
    let mut k = 0;
    let mut flag = false;
    for c in cc.iter().rev() {
        if flag {
            k -= c;
            flag = false;
        } else {
            k += c;
            flag = true;
        }
    }
    let mut ans = i128::MAX;
    for d1 in 0..m {
        let mut ds = vec![d1];
        let mut d = d1;
        for i in 1..n {
            d = (cc[i-1] - d) % m;
            while d < 0 {
                d += m;
            }
            ds.push(d);
        }
        let s = ds.iter().sum::<i128>();
        ans = ans.min(s);
    }
    println!("{}", ans);
}