fn main() {
    println!("{}", (sum_divisible_by(3, 1000) + sum_divisible_by(5, 1000) - sum_divisible_by(15, 1000)));
}

fn sum_divisible_by(n: u32, max: u32) -> u32 {
    let mut sum: u32 = 0;
    for number in 1..(max / n) {
        sum += number * n;
    }
    sum
}
