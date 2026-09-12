use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let s: usize = tokens.next().unwrap().parse::<usize>().unwrap() - 1;
    let l: u64 = tokens.next().unwrap().parse().unwrap();
    let a: Vec<u64> = tokens.map(|x| x.parse::<u64>().unwrap()).collect();
    let mut sums: Vec<u64> = vec![0];
    for i in 1..n {
        sums.push(sums[i - 1] + a[i - 1]);
    }
    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            if i <= s && s <= j {
                let x = (sums[s] - sums[i]).min(sums[j] - sums[s]);
                if x + sums[j] - sums[i] <= l {
                    ans = ans.max(j - i + 1);
                }
            }
        }
    }
    println!("{ans}");
}
