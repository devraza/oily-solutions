fn main() {
    let mut total: u64 = 0;
    let mut total1: u64 = 0;

    for i in 1..=100 {
        total += i * i;
        total1 += i;
    }

    let square_sum = total1.pow(2);
    let difference = square_sum - total;

    println!("{}", difference);
}
