
use std::io;

fn main() {        
    loop{ 
        println!("Please input first number.");

        let mut first_number = String::new();
        io::stdin()
            .read_line(&mut first_number)
            .expect("Failed to read line");

        let first_number: f32 = match first_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Please input second number.");

        let mut second_number = String::new();

        io::stdin()
            .read_line(&mut second_number)
            .expect("Failed to read line");

        let second_number: f32 = match second_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Please chose action: 1:\"+\", 2:\"-\", 3:\"/\", 4:\"*\""); 

        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");   

        let action: u8 = match action.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match action {
            1 => println!("{:?} + {:?} = {:?}",first_number, second_number, first_number + second_number),
            2 => println!("{:?} - {:?} = {:?}",first_number, second_number, first_number - second_number),
            3 => println!("{:?} / {:?} = {:?}",first_number, second_number, first_number / second_number),
            4 => println!("{:?} * {:?} = {:?}",first_number, second_number, first_number * second_number),
            _ => {
                println!("Wrong action!");
                break;
            }
        }

        break;
    }
    
}
