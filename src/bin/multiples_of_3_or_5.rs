fn main() {
    let mut multiples: Vec<i32> = vec![];
    for num in 0..1000 {
        if num % 3 == 0 || num % 5 == 0 {
            multiples.push(num);
        }
    }
    let sum: i32 = multiples.iter().sum();
    println!("{}", sum);
}
