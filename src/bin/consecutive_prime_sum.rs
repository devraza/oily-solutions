fn check_prime(x: u32) -> bool {
    let mut check = true;
    for j in 2..=(x as u32).isqrt() {
        if x % 2 == 0 {
            check = false;
        } else if x % j == 0 {
            check = false;
        }
    }
    check
}

fn consecutive_combinations<T: Clone>(vec: &[T]) -> Vec<Vec<T>> {
    let mut combinations = Vec::new();
    let len = vec.len();

    for size in 1..=len {
        for start in 0..=(len - size) {
            let window = vec[start..start + size].to_vec();
            combinations.push(window);
        }
    }

    combinations
}

fn main() {
    let mut primes: Vec<u32> = vec![];
    for i in 2..10000 {
        if check_prime(i) {
            primes.push(i);
        }
    }

    let mut maximum = 0;
    let mut sum = 0;

    let combinations = consecutive_combinations(&primes);

    for combination in combinations {
        let length = combination.len();
        let local_sum: u32 = combination.iter().sum();

        if check_prime(local_sum) && local_sum < 1000000 && length > maximum {
            maximum = length;
            sum = local_sum;
        }
    }

    println!("{}", sum);
}
