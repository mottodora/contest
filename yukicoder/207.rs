fn getline() -> String{
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}

fn have3(n:i32) -> bool {
    match n {
        a if a % 10 == 3 => true,
        a if a > 12     => have3(a/10),
        _               => false,
    }
}


fn main() {
    let s = getline();
    let v:Vec<_> = s.trim().split(' ').collect();
    let a:i32 = v[0].parse().unwrap();
    let b:i32 = v[1].parse().unwrap();
    for i in a..b+1 {
        if have3(i) || i%3==0 {
            println!("{}",i)
        }
    }
}
