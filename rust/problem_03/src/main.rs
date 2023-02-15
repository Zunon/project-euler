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
    let mut num: u64 = 600851475143;
    let mut max: u64 = 0;
    while num != 1 {
        for prime in (Prime { current: 1 }) {
            if num % prime == 0 {
                num /= prime;
                if prime > max {
                    max = prime;
                }
                break;
            }
        }
    }

    println!("{}", max);
}
