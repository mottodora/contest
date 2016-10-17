fn getline() -> String{
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn getintvec() -> Vec<i32> {
    let s = getline();
    s.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn main() {
    let v = getintvec();
    let d = v[0] as f32;
    let p = v[1] as f32;
    let r = ((d * (100.0 + p))/100.0).floor() as i32;
    println!("{}", r);
}
