use num_bigint::BigUint;

fn main() {
    let mut sums: Vec<u32> = vec![];

    for i in 1..100 {
        for j in 1..100 {
            let power = format!("{}", BigUint::from(i as u64).pow(j));
            sums.push(power.chars().filter_map(|c| c.to_digit(10)).sum());
       }
    }

    println!("{}", sums.iter().max().unwrap());
}
