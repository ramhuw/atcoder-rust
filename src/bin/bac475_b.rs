use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let _: usize = tokens.next().unwrap().parse().unwrap();
    let mut a0: usize = 0;
    let mut a1: usize = 0;
    let mut a2: usize = 0;
    for raw in tokens {
        let a: usize = raw.parse().unwrap();
        let mut charge = if a % 1000 == 0 {
            a
        } else {
            1000 * (a / 1000 + 1)
        } - a;
        let a2t = charge / 100;
        charge -= 100 * a2t;
        a2 += a2t;
        let a1t = charge / 10;
        charge -= a1t * 10;
        a1 += a1t;
        a0 += charge;
    }
    println!("{} {} {}", a0, a1, a2);
}
