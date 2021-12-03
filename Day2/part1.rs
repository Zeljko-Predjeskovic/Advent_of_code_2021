use std::fs;

fn main() {
    let data = fs::read_to_string("data_part1.txt")
    .expect("error reading");

    let c = data.lines();

    let mut horizontal = 0;
    let mut vertical = 0;

    for element in c{
        let el: Vec<String> = element.split(" ").map(|l| l.parse::<String>().unwrap()).collect();        
            if el[0] == "forward"{
                horizontal += el[1].parse::<i32>().unwrap();
            }
            if el[0] == "down"{
                vertical += el[1].parse::<i32>().unwrap();
            }
            if el[0] == "up"{
                vertical -= el[1].parse::<i32>().unwrap();
            }
    }

    println!("{}", horizontal*vertical);


}