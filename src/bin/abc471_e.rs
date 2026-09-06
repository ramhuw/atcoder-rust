use std::io::Read;
const P: usize = 998244353;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let k: usize = tokens.next().unwrap().parse().unwrap();
    let ball: Vec<usize> = tokens.map(|b| b.parse().unwrap()).collect();
    let mut fact: Vec<usize> = vec![1];
    for i in 1..n {
        fact.push(fact[i - 1] * i % P);
    }
    let mut sum: Vec<usize> = vec![0];
    for i in 0..(n - 1) {
        sum.push((sum[i] + ball[i]) % P);
    }
    let ans1: usize =
        ball.iter().fold(0, |acc, a| (acc + a * a % P) % P) * binom(n - 1, k - 1, &fact);
    let mut ans2: usize = 0;
    if k >= 2 {
        for j in (1..n).rev() {
            ans2 += 2 * ball[j] % P * sum[j] % P * binom(n - 2, k - 2, &fact) % P;
        }
    }
    println!("{}", (ans1 + ans2) % P);
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

fn binom(m: usize, n: usize, fact: &Vec<usize>) -> usize {
    fact[m] * inv(fact[n] * fact[m - n] % P) % P
}
