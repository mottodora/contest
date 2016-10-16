fn getline() -> String{
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}

fn count(l:i32, m:i32, n:i32) -> i32 {
    let m_2 = n / 25;
    let l_2 = (m + m_2) / 4;
    return n%25+(m+m_2)%4+(l+l_2)%10

}

fn main() {
    let s = getline();
    let l:i32 = s.trim().parse().unwrap();
    let s = getline();
    let m:i32 = s.trim().parse().unwrap();
    let s = getline();
    let n:i32 = s.trim().parse().unwrap();

    println!("{}", count(l, m, n));
}
