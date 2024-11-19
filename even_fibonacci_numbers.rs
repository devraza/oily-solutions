fn main() {
    let mut sequence: Vec<i32> = vec![1, 2];
    while sequence[sequence.len()-1] <= 4000000 {
        let mut last = sequence[sequence.len()-1];
        let mut secondlast = sequence[sequence.len()-2];
        (secondlast, last) = (last, secondlast+last);
        sequence.push(last);
    }
    for i in sequence.clone() {
        if i % 2 != 0 || i > 4000000 {
            sequence.remove(sequence.iter().position(|x| *x == i).expect("Item not found"));
        }
    }
    let sum: i32 = sequence.iter().sum();
    println!("{}", sum);
}
