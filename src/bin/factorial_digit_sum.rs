use num_bigint::BigUint;

fn factorial(x: u64) -> BigUint {
    let mut product = BigUint::from(1_u64);
    for i in 2..=x {
        product *= BigUint::from(i);
    }
    product
}

fn main() {
    let number = factorial(100);
    let mut digits: Vec<u64> = vec![];
    for i in number.to_string().chars().filter_map(|c| c.to_digit(10)) {
        digits.push(i as u64);
    }

    println!("{}", digits.iter().sum::<u64>());
}
