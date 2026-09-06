use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let l: Vec<usize> = tokens.map(|x| x.parse::<usize>().unwrap()).collect();
    let mut prefix_sum = vec![l[0]];
    for i in 1..n {
        prefix_sum.push(prefix_sum[i - 1] + l[i]);
    }
    let mut ans = usize::MAX;
    for i in 0..(n - 1) {
        let right = prefix_sum[n - 1] - prefix_sum[i];
        let diff = if prefix_sum[i] < right {
            right - prefix_sum[i]
        } else {
            prefix_sum[i] - right
        };
        ans = ans.min(diff);
    }
    println!("{ans}");
}
