use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let cs: Vec<String> = input.trim().chars().map(|x| x.to_string()).collect();
    let ans: String = cs.join("o");
    println!("{ans}");
}
