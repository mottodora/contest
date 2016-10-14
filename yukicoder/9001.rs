fn getline() -> String{
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}

fn main() {
    let s = getline();
    let a:Vec<_> = s.trim().split(' ').collect();
    let x:i32 = a[0].parse().unwrap();
    let y:i32 = a[1].parse().unwrap();
    print!("{} ", x+y);
    println!("{}", getline());
}
