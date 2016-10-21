fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    ret
}

fn main() {
    let a = getline();
    let b = getline();
    let mut x: Vec<char> = a.trim().chars().collect();
    let mut y: Vec<char> = b.trim().chars().collect();
    x.sort();
    y.sort();
    let s: String = x.into_iter().collect();
    let t: String = y.into_iter().collect();
    if s == t {
        println!("YES");
    } else {
        println!("NO");
    }
}

