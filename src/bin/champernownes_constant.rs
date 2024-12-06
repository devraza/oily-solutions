fn main() {
    let mut string: String = "0.".to_string();
    for i in 1..=185185 {
        string.push_str(&i.to_string());
    }
    
    let digits = vec![1, 10, 100, 1000, 10_000, 100_000, 1_000_000];
    let mut multiplying: Vec<u32> = vec![];

    for i in digits {
        let digit = &string.chars().nth(i+1);
        multiplying.push(digit.unwrap().to_digit(10).unwrap());
    }

    println!("{}", multiplying.iter().product::<u32>());
}
