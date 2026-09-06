fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim();
    if n == "1" {
        println!("2")
    } else if n == "2" {
        println!("3")
    } else if n == "3" {
        println!("2")
    }
}
