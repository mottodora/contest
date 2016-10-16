fn check(month:i32, day:i32) -> i32 {
    let x = day % 10 + day / 10;
    match x {
        a if a == month => return 1,
        _               => return 0,
    }
}

fn counthappyday(month:i32, day:i32) -> i32 {
    let mut count = 0;
    for i in 1..day+1 {
        count += check(month, i)
    }
    return count
}

fn happyday() -> i32 {
    let mut count = 0;

    for i in 1..13 {
        match i {
            2 => count += counthappyday(i, 28),
            4 | 6 | 9 | 11 => count += counthappyday(i, 30),
            _ => count += counthappyday(i, 31),
        }
    }
    return count
}

fn main() {
    println!("{}", happyday());
}
