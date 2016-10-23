use std::cmp::max;

fn getline() -> String{
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}

fn getvec(string: String) -> Vec<i32> {
    string.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
fn solve(t: i32) -> i32 {
    let v = getvec(getline());
    let mut table = [0; 1000];
    table[0] = v[0];
    if t > 1 { table[1] = max(v[0], v[1]);}
    for i in 2..t {
        let a = table[(i-1) as usize];
        let b = table[(i-2) as usize]+v[i as usize];
        table[i as usize] = max(a, b);
    }
    return table[(t-1) as usize]
}

fn main() {
    let t:i32 = getline().trim().parse().unwrap();
    println!("{}", solve(t));
}

