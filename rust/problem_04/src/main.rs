pub fn is_palindrome(n: i32) -> bool {
    let text = n.to_string();
    let rev = text.chars().rev().collect::<String>();
    text == rev
}

fn main() {
    let mut max = i32::MIN;
    for i in 100..1000 {
        for j in 100..1000 {
            let product = i * j;
            if is_palindrome(product) && product > max {
                max = product;
            }
        }
    }

    println!("{}", max);
}
