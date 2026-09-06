fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut hw = input.split_whitespace().map(|a| a.parse::<u32>().unwrap());
    let h = hw.next().unwrap();
    let w = hw.next().unwrap();
    let ans = if 10000 * w >= 25 * h * h {"Yes"} else {"No"};
    println!("{}", ans);
}
