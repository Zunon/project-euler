pub fn dividable_up_to(n: u32, max: u32) -> bool {
    for i in 1..max {
        if n % i != 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut n = 22;
    loop {
        if dividable_up_to(n, 20) {
            println!("{}", n);
            break;
        }
        n += 2;
    }
}
