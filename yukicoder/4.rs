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

fn main() {
    //let n:i32 = getline().trim().parse().unwrap();
    let _ = getline();
    let ws = getvec(getline());
    let sum_w = ws.iter().fold(0, |x, y| x + y);
    let mut dp = vec![false; 5100];
    dp[0] = true;
    if sum_w % 2 == 0 {
        for w in ws {
            for i in (0..sum_w/2).rev() {
                if dp[i as usize] {
                    dp[(i+w) as usize] = true;
                }
            }
        }
        if dp[(sum_w/2) as usize] {
            println!("possible");
        }
        else {
            println!("impossible");
        }
    }
    else {
        println!("impossible");
    }
}

