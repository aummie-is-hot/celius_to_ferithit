/*
By: <Aum markandey>
Date: 2026-09-28
Program Details: <celiuus to fahrenheit converter>
*/

fn main() {
     use std::io;
    println!("pick celius or fahrenheit using the number 1 for celius and 2 for fahrenheit: ");
     let mut corf = String::new();
    io::stdin().read_line(&mut corf).expect("Failed to read line");
    println!("Enter a value to convert:");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line");
    let corf = corf.trim().parse::<f64>();
let value = value.trim().parse::<f64>();
    if let (Ok(corf), Ok(value)) = (corf, value) {
    if corf==1.0 {
        let fahrenheit = (value * 9.0 / 5.0) + 32.0;
        println!("{} Celsius is equal to {} Fahrenheit", value, fahrenheit);
    } else if corf==2.0 {
        let celsius = (value - 32.0) * 5.0 / 9.0;
        println!("{} Fahrenheit is equal to {} Celsius", value, celsius);
    } else {
        println!("Invalid input. Please enter 1 for Celsius or 2 for Fahrenheit.");
    }
    
    
}
else {
    
    println!("Fuck you, you entered invalid numbers. Please enter valid numbers next time.");
}    
}            
