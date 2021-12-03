use std::fs;

fn main() {
    let data = fs::read_to_string("data_part2.txt")
    .expect("error reading");

    let c = data.lines();

    let mut horizontal: i64 = 0;
    let mut vertical: i64 = 0;
    let mut aim: i64 = 0;

    for element in c{
        let el: Vec<String> = element.split(" ").map(|l| l.parse::<String>().unwrap()).collect();  
        let number = el[1].parse::<i64>().unwrap();    
            if el[0] == "forward"{
                horizontal += number;
                vertical += aim*number;
                }
            if el[0] == "down"{
                aim += number;
            }
            if el[0] == "up"{
                aim -= number;
            }
    }

    println!("{}", horizontal*vertical);


}