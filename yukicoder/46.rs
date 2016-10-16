fn getline() -> String{
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}

fn walk(a:i32, b:i32) -> i32 {
    match a % b {
        0 => a / b,
        _ => a / b + 1
    }
}

fn main() {
    let s = getline();
    let a:Vec<_> = s.trim().split(' ').collect();
    let x:i32 = a[0].parse().unwrap();
    let y:i32 = a[1].parse().unwrap();
    println!("{}", walk(y, x));
}
