use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|x| x.parse::<u8>().unwrap());
    let a = tokens.next().unwrap();
    let b = tokens.next().unwrap();
    if a.saturating_add(b) == 9 || (a.saturating_sub(b) == 9) || a.saturating_mul(b) == 9 || (a == 9 * b) {
        println!("Nine");
    } else {
        println!("Nein");
    }
}