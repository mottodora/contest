fn getline() -> String{
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}

fn eratosthenes(n: i32) -> Vec<i32> {
    let mut v: Vec<i32> = (2..n+1).collect();
    let mut i:i32 = 0;
    while i < v.len() as i32 {
        let a = v[i as usize];
        v.retain(|&x| x< a+1 || x % a !=0);
        i+=1;
    }
    v
}

fn main() {
    let n: i32 = getline().trim().parse().unwrap();
    let primes = eratosthenes(n);
    let mut dp = [false; 10001];
    let l = primes.len() as i32;

    for i in 4..(n+1) {
        let mut j = 0;
        while primes[j as usize] <= i-2{
            dp[i as usize] |= !dp[(i - primes[j as usize]) as usize];
            j += 1;
            if j >= l {
                break;
            }
        }
    }
    if dp[n as usize] {
        println!("Win");
    }
    else {
        println!("Lose");
    }
}
