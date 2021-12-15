fn main() {
    loop {
        clearscreen::clear().unwrap();
        let choice = main_menu(false);
        clearscreen::clear().unwrap();
        match choice {
            1 => {
                loop {
                    println!("To go back to main menu, enter 'q'");
                    println!("Enter a temperature in Fahrenheit:");

                    let mut fahrenheit = String::new();
                    std::io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");

                    if fahrenheit.trim() == "q" {
                        println!("Going back to main menu...");
                        clearscreen::clear().unwrap();
                        break;
                    }

                    let fahrenheit: f32 = match fahrenheit.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            clearscreen::clear().unwrap();
                            println!("Invalid input. Please enter a number.");
                            println!("----------------------------------------");
                            continue
                        },
                    };
                    clearscreen::clear().unwrap();
                    println!("{:.2}°F is {:.2}°C", fahrenheit, fahrenheit_to_celsius(fahrenheit));
                    println!("----------------------------------------");
                }
            },
            2 => {
                loop {
                    println!("To go back to main menu, enter 'q'");
                    println!("Enter a temperature in Celsius:");
                    let mut celsius = String::new();
                    std::io::stdin().read_line(&mut celsius).expect("Failed to read line");

                    if celsius.trim() == "q" {
                        println!("Going back to main menu...");
                        clearscreen::clear().unwrap();
                        break;
                    }

                    let celsius: f32 = match celsius.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            clearscreen::clear().unwrap();
                            println!("Invalid input. Please enter a number.");
                            println!("----------------------------------------");
                            continue;
                        }
                    };
                    clearscreen::clear().unwrap();
                    println!("{:.2}°C is {:.2}°F", celsius, celsius_to_fahrenheit(celsius));
                    println!("----------------------------------------");
                }

            }
            0 => {
                println!("Exiting...");
                break;
            }
            _ => {
                clearscreen::clear().unwrap();
                println!("Invalid choice!");
                continue;
            }
        }
    }
}

fn main_menu(break_on_invalid: bool) -> i8 {
    loop {
        println!("Choose a conversion:");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("--------------------------------");
        println!("0. Exit");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim().parse::<i8>() {
            Ok(num) => {
                clearscreen::clear().unwrap();
                return num
            },
            Err(_) => {
                clearscreen::clear().unwrap();
                println!("Invalid option. Please enter a number.");
                if break_on_invalid {std::process::exit(1)} else {continue}
            }
        }
    }
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    temp * 1.8 + 32.0
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.0) / 1.8
}