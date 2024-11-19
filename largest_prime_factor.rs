fn main() {
    let mut num: u64 = 600851475143;
    let mut factors: Vec<u64> = vec![];
    let mut i: u64 = 2;
    while i.pow(2) <= num {
            if num % i == 0 {
                factors.push(i);
                num = (num as f64 / i as f64).floor() as u64;
            } else {
                i += 1;
            }
    }
    if num != 1 {
        factors.push(num);
    }

    println!("{:?}", factors);
}
