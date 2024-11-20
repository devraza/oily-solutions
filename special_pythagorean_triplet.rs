fn main() {
    'algorithm: loop {
        for i in 1..1000 {
            for j in 1..1000 {
                for k in 1..1000 {
                    let pythagoras: f64 = (i as f64).powf(2.) + (j as f64).powf(2.);
                    if pythagoras == (k as f64).powf(2.) {
                        if i+j+k == 1000 {
                            println!("{}", i*j*k);
                            break 'algorithm;
                        }
                    }
                }
            }
        }
    }
}
