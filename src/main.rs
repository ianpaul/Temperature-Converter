use std::io;

fn main() {
    println!("Is your temperature in Fahrenheit or Celsius?");
    let mut deg = String::new();
    io::stdin().read_line(&mut deg).expect("Failed to read line.");

    if deg.trim() == "F" || deg.trim() == "Fahrenheit" {
        println!("Enter your temperature in Fahrenheit:");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read line.");
        let temp:f64 = temp.trim().parse().expect("Please type a number!");
        let result = (temp - 32.0) * 5.0/9.0;
        if result > 29.0 {
            println!("Your temperature in Celsius is: {:.1}\u{00B0}C \u{1F975}", result);
        } else if result < 10.0 {
            println!("Your temperature in Celsius is: {:.1}\u{00B0}C \u{1F976}", result);
        } else {
            println!("Your temperature in Celsius is: {:.1}\u{00B0} \u{1F604}", result);
        }
     } else if deg.trim() == "C" || deg.trim() == "Celsius" {
        println!("Enter your temperature in Celsius:");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read line.");
        let temp:f64 = temp.trim().parse().expect("Please type a number!");
        let result = (temp * 9.0/5.0) + 32.0;
        if result > 84.0 {
            println!("Your temperature in Fahrenheit is: {:.1}\u{00B0}F \u{1F975}", result);
        } else if result < 10.0 {
            println!("Your temperature in Fahrenheit is: {:.1}\u{00B0}F \u{1F976}", result);
        } else {
            println!("Your temperature in Fahrenheit is: {:.1}\u{00B0}F \u{1F604}", result);
        }
    } else {
        println!("You didn't enter a valid response please enter Fahrenheit, F, Celsius, or C");
    }
    
}
