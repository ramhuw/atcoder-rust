use std::io::Read;
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let n = tokens.next().unwrap();
    let mut counter: usize = 0;
    let mut cycle: usize = 1;
    for pi in tokens {
        if pi > cycle * 10 {
            println!("No");
            return;
        }
        counter += 1;
        if counter == 10 {
            counter -= 10;
            cycle += 1;
        }
    }
    println!("Yes")
}
