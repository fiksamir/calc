
use std::io;
use std::io::Write;

#[derive(Debug)]
enum Actions {
    Add,
    Substr,
    Mult,
    Div,
}

fn get_result(x:f64, y:f64, action:Actions) -> Result<f64, &'static str>{

    match action {
        Actions::Add => Ok(x + y),
        Actions::Substr => Ok(x - y),
        Actions::Mult => Ok(x * y),
        Actions::Div => {
            if y == 0.0 {
                Err("Division by zero")
            } else {
                Ok(x / y)
            }
        }           
    }
}

fn main() {        
    loop{ 
        print!("Please input first number:");

        let _ = io::stdout().flush();
        let mut x = String::new();

        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read line");
            

        let x: f64 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error, this is not a number.");
                continue
            }
        };
        print!("Please input second number:");

        let _ = io::stdout().flush();
        let mut y = String::new();

        io::stdin()
            .read_line(&mut y)
            .expect("Failed to read line");

        let y: f64 = match y.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error, this is not a number.");
                continue
            }
        };

        print!("Please choose action: \"+\", \"-\", \"/\", \"*\":"); 

        let _ = io::stdout().flush();
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");  
        let action = action.trim(); 

        match action {
            "+" => match get_result(x, y, Actions::Add)  {
                Ok(num) => println!("{} + {} = {}",x, y, num),
                Err(e) => println!("{}", e),                
            }, 
            "-" => match get_result(x, y, Actions::Substr)  {
                Ok(num) => println!("{} + {} = {}",x, y, num),
                Err(e) => println!("{}", e),                
            }, 
            "/" => match get_result(x, y, Actions::Div)  {
                Ok(num) => println!("{} + {} = {}",x, y, num),
                Err(e) => println!("{}", e),                
            }, 
            "*" => match get_result(x, y, Actions::Mult)  {
                Ok(num) => println!("{} + {} = {}",x, y, num),
                Err(e) => println!("{}", e),                
            }, 
            _ => {
                println!("Wrong action!");
                continue;
            }
        }
        break;
    }
    
}
