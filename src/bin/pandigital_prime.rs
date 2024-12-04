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

fn permutations(input: &Vec<u64>) -> Vec<u64> {
    let mut permutations = vec![];
    for permutation in input.iter().permutations(input.len()).unique() {
        let result: u64 = permutation.iter().fold(0, |acc, &x| acc * 10u64.pow(x.to_string().len() as u32) + x);
        permutations.push(result);
    }

   permutations

}

fn main() {
    let mut pandigitals: Vec<u64> = vec![];
    let mut permutator: Vec<u64> = vec![];

    for i in 1..=9 {
        permutator.push(i); 

        for j in permutations(&permutator) {
            if is_prime(j) {
                pandigitals.push(j);
            }
        }
    }

    println!("{}", pandigitals.iter().max().unwrap());
}
