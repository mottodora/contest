fn getline() -> String{
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}

fn extract_int_sum(string: &str) -> i32 {
    string.chars().filter(|x| x.is_numeric())
        .map(|x| x.to_string().parse::<i32>().unwrap())
        //.sum()
        .fold(0, |sum, i| sum+i)
}

fn main() {
    let s = getline();
    println!("{}", extract_int_sum(s.trim()));
}

