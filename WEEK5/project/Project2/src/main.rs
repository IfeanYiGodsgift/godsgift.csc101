/* Rust program to determine incentive of employees
Property of GODSGIFT IFEANYI*/ 

use std::io;

fn main() {
    let mut a:i32 = 0;
    let mut b = String::new();
    
    let exp = "yes";
    let ixp = "no";

    let n1 = 1_560_000;
    let n2 = 1_480_000;
    let n3 = 1_300_000;
    let n4 = 100_000;
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("Enter Age: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    a = input1.trim().parse().expect("Not a valid number");


    println!("Are you experienced(yes/no): ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    b = input2.trim().parse().expect("Not a valid level");


    if a >=40  && b == exp
    {

        println!(" You are entitled to : {}", n1);
        
    }
    else 
    {
 
        if a >= 30 &&  b == exp 
        {

            println!("You are entitled to : {}", n2);

        }
        else if a < 28  &&  b == exp
        {
            println!("You are entitled to : {}", n3);
        }
        else if a > 18  &&  b == ixp
        {
            println!("You are entitled to : {}", n4);
        }
    }
}
