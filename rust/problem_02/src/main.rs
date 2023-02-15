struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn main() {
    let sum: u32 = Fibonacci { curr: 0, next: 1 }
        .take_while(|&x| x < 4_000_000)
        .filter(|&x| x % 2 == 0)
        .sum();

    println!("{}", sum);
}
