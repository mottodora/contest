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

fn bubblesort(v: &Vec<i32>) -> Vec<i32> {
    let mut dst = v.to_owned();
    for i in 0..dst.len() {
        for j in i..dst.len() {
            if dst[i] > dst[j] {
                dst.swap(i, j)
            }
        }
    }
    return dst
}

fn median_odd(v: &Vec<i32>) -> i32 {
    v[v.len()/2]
}

fn median_even(v: &Vec<i32>) -> f32 {
    let m = v[v.len()/2-1] as f32;
    let n = v[v.len()/2] as f32;
    (n+m)/2.0
}



fn main() {
    let _ = getline();
    let nums = getline();
    let nums = getvec(nums);
    let sorted = bubblesort(&nums);
    match sorted {
        ref v if v.len() % 2 == 0 => println!("{}", median_even(&sorted)),
        _                         => println!("{}", median_odd(&sorted))
    }
}

