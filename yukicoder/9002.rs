fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}

fn fizzbuzz(n: i32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (_, 0) => "Buzz".to_string(),
        (0, _) => "Fizz".to_string(),
        _      => n.to_string()
    }
}

fn main() {
    let s = getline();
    let x:i32 = s.trim().parse().unwrap();
    for i in 1..x+1 {
        println!("{}", fizzbuzz(i))
    }
}
