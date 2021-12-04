use std::fs;

fn main() {
    let data = fs::read_to_string("data_part1.txt")
    .expect("error reading");

    let c: Vec<String> = data.lines().map(|l| l.parse::<String>().unwrap()).collect(); //Jede Zeile in ein Vec

    let mut x = "".to_string();
    let mut y = "".to_string();
    let mut count = 0;


        //laenge einer Zeile 
        for el1 in 0..(c[0].len()){

            //länge aller Zeilen
            //Geht durch den el1 position durch alle Zeilen und schaut ob es mehr 1er gibt als 0er, in count gespeichert
            for el2 in 0..c.len(){

                let f: String = c[el2].chars().nth(el1).unwrap().to_string(); //el1 index von el2 Zeile

                if f== "1"{
                count += 1;
            }
        }
        //wen mehr als die hälfte der Zeilen 1 ist gibt es mehr 1er, zB 1000 Zeilen, count = 501, mehr 1er in der Zeile
        if count > c.len()/2{
            x += "1";
            y += "0";
        }
        else{
            y += "1";
            x += "0";
        }
        count = 0;
    }
     //binary string in int umwandeln
     let a = isize::from_str_radix(&x, 2).unwrap();
     let b = isize::from_str_radix(&y, 2).unwrap(); 
    println!("{}",a*b);

}