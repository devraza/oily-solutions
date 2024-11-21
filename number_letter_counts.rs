const GENERIC: [&str; 20] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const SPECIAL: [&str; 11] = [
    "",
    "",
    "twenty",
    "thirty",
    "forty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
    "hundred",
]; 

fn translate(number: u64) -> String {
    let mut result: String = "".to_string();

    match number {
        number if number < 20 => { result = GENERIC[(number) as usize].to_string(); },
        number if number >= 20 && number < 100 => {
            let number_string = number.to_string();
            let tens = number_string.chars().nth(0).unwrap().to_digit(10).unwrap();
            let ones = number_string.chars().nth(1).unwrap().to_digit(10).unwrap();

            result = SPECIAL[tens as usize].to_owned()+GENERIC[(ones) as usize];
        },
        100 => { result = "onehundred".to_string(); }
        number if number > 100 && number < 1000 => {
            let number_string = number.to_string();
            let hundreds = number_string.chars().nth(0).unwrap().to_digit(10).unwrap();
            let tens = number_string.chars().nth(1).unwrap().to_digit(10).unwrap();
            let ones = number_string.chars().nth(2).unwrap().to_digit(10).unwrap();

            let combined = format!("{}{}", number_string.chars().nth(1).unwrap(), number_string.chars().nth(2).unwrap()).parse::<u64>().unwrap();

            if combined < 20 {
                if ones == 0 && tens == 0 {
                    result = GENERIC[hundreds as usize].to_owned()+"hundred"+GENERIC[combined as usize];
                } else {
                    result = GENERIC[hundreds as usize].to_owned()+"hundredand"+GENERIC[combined as usize];
                }
            } else {
                result = GENERIC[hundreds as usize].to_owned()+"hundredand"+SPECIAL[(tens) as usize]+GENERIC[(ones) as usize];
            }
        },
        1000 => { result = "onethousand".to_string(); }
        _ => {},
    }

    result
}

fn main() {
    let mut total = 0;

    for i in 1..=1000 {
        total += translate(i).len();
    }
 
    println!("{}", total);
}
