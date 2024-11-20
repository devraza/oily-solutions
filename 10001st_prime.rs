fn is_prime(num: u64) -> bool {
    let prime: bool = true;
    match num {
        0..=1 => false,
        2 => true,
        num if num % 2 == 0 => false,
        _ => {
            for i in (3..=((num as f64).sqrt() as u64)).step_by(2) {
                if num % i == 0 {
                    return false;
                }
            }
            prime
        }
    }
}

fn main() {
    let mut primes: Vec<u64> = vec![];
    let mut increment: u64 = 2;

    while primes.len() < 10001 {
        if is_prime(increment) {
            primes.push(increment);
        }
        increment += 1;
    }

    println!("{:?}", primes[primes.len()-1]);
}
