use itertools::Itertools;

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

fn permutations(input: &[u64]) -> Vec<u64> {
    let mut permutations = vec![];
    for permutation in input.iter().permutations(input.len()).unique() {
        let result: u64 = permutation.iter().fold(0, |acc, &x| acc * 10u64.pow(x.to_string().len() as u32) + x);
        permutations.push(result);
    }

    permutations
}

fn main() {
    let mut primes: Vec<u64> = vec![];

    for i in 1000..=10000 {
        if is_prime(i) {
            primes.push(i);
        }
    }

    let mut values: Vec<String> = vec![];

    for i in primes {
        let mut prime_permutations: Vec<u64> = vec![];

        let i_vector: Vec<u64> = i.to_string().chars().filter_map(|c| c.to_digit(10).map(|d| d as u64)).collect();
        for j in permutations(&i_vector) {
            if is_prime(j) {
                prime_permutations.push(j);
            }
        }

        prime_permutations.sort();
        let prime_permutations: Vec<u64> = prime_permutations.into_iter().filter(|x| *x != i).collect();

        for j in &prime_permutations {
            if *j < 1000 {
                break;
            }

            for k in prime_permutations.clone().into_iter().filter(|x| *x != *j).collect::<Vec<u64>>() {
                if (*j as i64-i as i64).abs() == (k as i64-i as i64).abs() {
                    let difference = (*j as i64 - i as i64).unsigned_abs();
                    let final_vector: Vec<u64> = vec![i-difference, i, i+difference];

                    let concat = final_vector.into_iter().map(|s| s.to_string()).collect::<String>();

                    if !values.contains(&concat) {
                        values.push(concat);
                    }
                }
            }
        }
    }

    for i in &values {
        println!("{}", i);
    }
}
