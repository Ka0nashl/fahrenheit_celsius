use std::io;

fn main() {
    
    println!("Welcome to the temperature converter !");

    'outer: loop {

        println!("Please choose from which unit you want to convert :\n(1) Fahrenheit to Celsius\n(2) Celsius to Fahrenheit\nOr if you want to leave : (L)");

        let mut f_c = String::new();

        io::stdin()
            .read_line(&mut f_c)
            .expect("Failed to read line");

        let f_c = f_c.trim().to_uppercase();
        
        if f_c == "L" {
            break
        }

        println!("What temperature?");

        let mut original_temp= String::new();

        io::stdin()
            .read_line(&mut original_temp)
            .expect("Failed to read line");

        let original_temp : i32 = original_temp.trim().parse().expect("Please type a number!");

        if f_c == "1" {
            if original_temp < -460 {
                println!("This temperature doesn't exist!");
                continue
            } else {
                let final_temp : i32 = (original_temp - 32)/9*5;
                println!("{} fahrenheit is {} celsius!", original_temp, final_temp);
            }
        } else if f_c == "2" {
            if original_temp < -273 {
                println!("This temperature doesn't exist!");
                continue
            } else {
                let final_temp : i32 = original_temp*9/5+32;
                println!("{} celsius is {} fahrenheit", original_temp, final_temp);
            }
        } else {
            println!("You have to choose either 1 or 2!");
            continue
        }
        
        loop {

            println!("Do you wish to restart the convertor ?\nY/N ?");
        
            let mut restart = String::new();

            io::stdin()
                .read_line(&mut restart)
                .expect("Failed to read line");

            let restart = restart.trim().to_uppercase();
            
            if restart == "Y" {
               continue 'outer;
            } else if restart == "N" {
                break 'outer;
            } else {
                println!("You can only say Yes or No.");
            }
        }
    }
}
