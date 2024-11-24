use num_bigint::BigUint;

fn main() {
    let mut values: Vec<BigUint> = vec![];
    for i in 1..=1000 {
        values.push(BigUint::from(i).pow(i));
    }

    let sum = values.iter().sum::<BigUint>().to_string();
    let last = &sum[(sum.len()-10)..sum.len()];

    println!("{}", last);
}
