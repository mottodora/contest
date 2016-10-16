fn getline() -> String{
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}

fn capslock(c: char) -> String {
    match c {
        c if c.is_uppercase() => c.to_string().to_lowercase(),
        _                     => c.to_string().to_uppercase(),
    }
}

fn convert(string: &str) -> String {
    string.chars().map(capslock).collect::<String>()
}

fn main() {
    let s = getline();

    println!("{}", convert(s.trim()));
}

