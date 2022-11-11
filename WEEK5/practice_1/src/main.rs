//property of GODSGIFT IFEANYI
use std::io;

fn main() {
    println!("\nStudent information Management Systrem !");

    //input name
    println!("\nPlease enter your name.");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    println!("Your Name is {}", name);
    
    //input age
    println!("\nEnter your age.");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("failed to red input");
    let age:i32 = age.trim().parse().expect("Input not an Integr");
    println!("Your age is: {}", age);   


}

