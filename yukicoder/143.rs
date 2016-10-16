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
    let k = v[0];
    let n = v[1];
    let families = getintvec();
    let years = families.iter().fold(0, |acc, x| acc + x);
    let num = k * n - years;
    if num >= 0 {
        println!("{}", num);
    } else {
        println!("-1");
    }
}

