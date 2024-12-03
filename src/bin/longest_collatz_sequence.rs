fn collatz_sequence(starting_num: u64) -> Vec<u64> {
    let mut num = starting_num;
    let mut result: Vec<u64> = vec![];

    result.push(num);

    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
            result.push(num);
        } else {
            num = (3*num)+1;
            result.push(num);
        }
    }

    result
}

fn main() {
    let mut increment: u64 = 0;
    let mut longest: u64 = 0;

    for i in 1..=1000000 {
        let length = collatz_sequence(i).len() as u64;
        if length > longest {
            longest = length;
            increment = i;
        }
    }

    println!("{:?}", increment);
}
