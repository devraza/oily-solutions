use num_bigint::BigUint;

fn main() {
    let mut number: BigUint = BigUint::from(2 as u32);
    number = number.pow(1000);

    let number_list: Vec<u32> = number
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u32))
        .collect();
    let sum: u32 = number_list.iter().sum();

    println!("{}", sum);
}
