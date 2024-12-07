use num_bigint::BigUint;

fn main() {
    let mut count = 0;
    for i in 1..=9 {
        for j in 1..=100 {
            let length = BigUint::from(i as u64).pow(j).to_string().len();
            if length == j.try_into().unwrap() {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
