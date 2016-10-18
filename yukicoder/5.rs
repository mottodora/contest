fn getline() -> String{
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    ret
}

fn getvec(string: String) -> Vec<i32> {
    string.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn solve(v: &Vec<i32>, l: i32) -> usize {
    let mut acc = 0;
    for (i, a) in v.into_iter().enumerate() {
        acc = acc + a;
        if acc > l {
            return i
        }
        else if acc == l {
            return i+1
        }
    }
    v.len()
}



fn main() {
    let l:i32 = getline().trim().parse().unwrap();
    let _:i32 = getline().trim().parse().unwrap();
    let mut v = getvec(getline());
    v.sort();
    println!("{}", solve(&v, l))
}

