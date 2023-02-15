struct Prime {
    current: u64,
}

impl Iterator for Prime {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.current += 1;
        while !is_prime(self.current) {
            self.current += 1;
        }
        Some(self.current)
    }
}

pub fn is_prime(n: u64) -> bool {
    if n == 2 {
        true
    } else if n % 2 == 0 || n < 2 {
        false
    } else {
        for i in 3..f64::floor(f64::sqrt(n as f64)) as u64 + 1 {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut prime = Prime { current: 1 };
    (0..10001)
        .into_iter()
        .for_each(|_| {prime.next();});
    println!("{}", prime.current);
}
