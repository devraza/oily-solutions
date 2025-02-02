use std::fs;

fn main() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut characters = vec![];

    for c in alphabet.chars() {
        characters.push(c);
    }
    
    let mut contents = fs::read_to_string("assets/input")
        .expect("Could not read the input file").replace("\"\n", "");
    if !contents.is_empty() {
        contents.remove(0);
    }
    let words = contents.split("\",\"").collect::<Vec<&str>>();

    let mut triangles = 0;
    for word in &words {
        let mut total_index = 0;
        for character in word.chars() {
            let index = characters.iter().position(|c| *c == character).unwrap()+1;
            total_index += index;
        }

        let position: f64 = (-1_f64+((1+8*total_index) as f64).sqrt())/2.;
        if position.fract() == 0. {
            triangles += 1;
        }
    }

    println!("{}", triangles);
}
