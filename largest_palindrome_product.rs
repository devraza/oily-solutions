fn main() {
    let mut product: i64 = 0;

    for i in 1..=999 {
        for j in 1..=999 {
            let product_string: String = format!("{:?}", i * j);
            let reversed_string = product_string.chars().rev().collect::<String>();

            if i * j > product && reversed_string == product_string {
                product = i * j;
            }
        }
    }

    println!("{}", product);
}
