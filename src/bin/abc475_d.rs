use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let s: Vec<char> = input.trim().chars().collect();
    let mut d: Vec<bool> = vec![true; 10000000];
    d[0] = false;
    d[1] = false;
    'outer: for p in 2..10000000 {
        if d[p] {
            let t: Vec<char> = p.to_string().chars().collect();
            if t.len() == s.len() {
                for i in 0..s.len() {
                    for j in (i + 1)..s.len() {
                        if s[i] == s[j] && t[i] != t[j] || t[i] == t[j] && s[i] != s[j] {
                            continue 'outer;
                        }
                    }
                }
                println!("{p}");
                return;
            }
            for q in ((2 * p)..10000000).step_by(p) {
                d[q] = false;
            }
        }
    }
    println!("-1");
}
