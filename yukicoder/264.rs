fn getline() -> String{
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}

fn janken(n: i32, k: i32) -> String{
    let a = (n - k + 3) % 3;
    match a {
        0 => "Drew".to_string(),
        1 => "Lost".to_string(),
        _ => "Won".to_string(),
    }
}

fn main() {
    let s = getline();
    let a:Vec<_> = s.trim().split(' ').collect();
    let n:i32 = a[0].parse().unwrap();
    let k:i32 = a[1].parse().unwrap();
    println!("{}", janken(n, k));
}
