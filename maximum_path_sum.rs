const TRIANGLE: &str =
"75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

fn main() {
    let triangle_one: Vec<&str> = TRIANGLE.split("\n").collect();
    let mut triangle_two: Vec<Vec<u32>> = vec![];

    for i in triangle_one {
        triangle_two.push(
            i.split(' ')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect());
    }

    let mut sums: Vec<u32> = vec![];
    let mut current_index: u32 = 0;

    sums.push(triangle_two[0][0]);
    triangle_two.remove(0);

    for i in triangle_two {
        let compare = vec![i[current_index as usize], i[(current_index+1) as usize]];
        let maximum = *compare.iter().max().unwrap();

        if maximum == compare[1] {
            current_index += 1;
        }

        sums.push(maximum);
    }    

    println!("{}", sums.iter().sum::<u32>());
}
