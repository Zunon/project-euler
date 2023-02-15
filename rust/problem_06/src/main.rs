fn main() {
    let sum_of_squares: u32 = (1..101).map(|x| x * x).sum();
    let square_of_sum: u32 = (1..101).sum::<u32>().pow(2);
    println!("{}", square_of_sum - sum_of_squares);
}
