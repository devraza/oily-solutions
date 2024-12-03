fn main() {
    let mut numerator: f64 = 1.;

    'outer: loop {
        for i in 1..=20 {
            if numerator % (i as f64) != 0. {
                numerator += 1.;
                continue 'outer;
            }
        }
        break;
    }

    println!("{}", numerator);
}
