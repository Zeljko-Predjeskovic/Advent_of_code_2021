use std::fs;

fn main() {
    let data = fs::read_to_string("data_part1.txt")
    .expect("error reading");

    let c: Vec<i64> = data
            .lines()
            .map(|l| l.parse::<i64>().unwrap())
            .collect();

    let mut count = 0;

    for element in 1..c.len(){
        if c[element] > c[element-1]{
            count += 1;
        }
    }

    println!("{}",count);


}