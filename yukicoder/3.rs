use std::collections::{HashMap,VecDeque};

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}

fn subbinary(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n % 2 == 0 {
        subbinary(n/2)
    } else {
        subbinary(n/2) + 1
    }
}

fn main() {
    let t:i32 = getline().trim().parse().unwrap();
    let mut map = HashMap::new();
    let mut buf = VecDeque::new();
    map.insert(1,1);
    buf.push_back((1, 1));
    let mut result = -1;
    while !buf.is_empty() {
        let (now, walk) = buf.pop_front().unwrap();
        //let now = tmp.0;
        //let walk = tmp.1;
        if now == t {
            result = walk;
            break
        }
        let d = subbinary(now);
        if !map.contains_key(&(now-d)) && now-d<=t && now-d>0 {
            map.insert(now-d,walk+1);
            buf.push_back((now-d,walk+1));
        }
        if !map.contains_key(&(now+d)) && now+d<=t && now+d>0 {
            map.insert(now+d,walk+1);
            buf.push_back((now+d,walk+1));
        }
    }
    println!("{}",result);
}

