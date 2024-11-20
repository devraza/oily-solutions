pub fn factors(num: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    for i in 1..((num as f64).sqrt() as u64 + 1) {
        if num % i == 0 {
            factors.push(i);
            if i != num / i {
                factors.push(num / i);
            }
        }
    }
    factors
}

fn main() {
    let mut triangular_increment: u64 = 0;
    let mut triangular: u64 = 0;
   
    while factors(triangular).len() <= 500 {
        triangular = 0;
        for i in 1..=triangular_increment {
            triangular += i;
        }
        triangular_increment += 1;
    }
    println!("{}", triangular);
}
