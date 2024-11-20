fn is_prime(num: u64) -> bool {
    let prime: bool = true;
    for i in 2..=((num as f64).sqrt() as u64) {
        if num % i == 0 {
            return false;
        }
    }
    prime
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

    println!("{:?}", primes);
}
