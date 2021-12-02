use std::fs;

fn main() {
    let data = fs::read_to_string("data_part2.txt")
    .expect("error reading");

    let c: Vec<i64> = data
            .lines()
            .map(|l| l.parse::<i64>().unwrap())
            .collect();

    let mut count = 0;

    for element in 3..c.len(){
        let x = c[element] + c[element-1] + c[element-2];
        let y = c[element-3] + c[element-1] + c[element-2];
        if x > y{
            count += 1;
        }
    }

    println!("{}",count);


}