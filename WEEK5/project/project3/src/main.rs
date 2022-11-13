/*Rust program to create a menu of the DGD
    Property of GODSGIFT IFEANYI*/
use std::io;
use std::collections::HashMap;
        //Hashmap was inspired by stackoverflow
        //it's like the lookup function its subjects the 
        // characters to the numbers 
        // so p as a character would be set at 3200
        // remember to always use it with the use std::collections::Hashmap crate
        // or it wont work
        // keep in mind for later use


fn display(){
    /* found out you dont have to necceserily tipe in the
       main function, u can just type a different function 
       and call that function in the main one
       saves us stress
       keep in mind for later use
    */
    println!("\n\t\tDOUBLE Gs Dinner\n");
    println!("\t\tWelcome to the DGD what can we get yah");
    println!("P. Poundo Yam/Edikaiko Soup\t-N3,200\nF. Fried Rice & Chicken\t\t-N3,000\nA. Amala & Ewedu Soup\t\t-N2,500\nE. Eba & Egusi Soup\t\t-N2,000\nW. White Rice & Stew\t\t-N2,500\n");
    println!("Enter 'n' to end order", );
}


fn get_customer_order(){
    println!("What would you like today? : ");
    
    let mut users_choice:char = ' ';
    let menu = HashMap::from([
        ('p', String::from("Poundo Yam/Edikaiko Soup")), 
        ('f', String::from("Fried Rice & Chicken")),
        ('a', String::from("Amala & Ewedu Soup")),
        ('e', String::from("Eba & Egusi Soup")),
        ('w', String::from("White Rice & Stew"))
        ]);


    let price = HashMap::from([ 
        ('p', 3200),
        ('f',3000),
        ('a',2500),
        ('e',2000),
        ('w',2500),
        ]);

    let mut total_price: f32 = 0.0;

    while users_choice != 'n' {
        
        let mut quantity: i32 = 0;
        let mut input1 = String::new();
        let mut input2 = String::new();
        

        io::stdin().read_line(&mut input1).expect("Failed to understand");
        users_choice = input1.trim().parse().expect("Not an integer value");
        if users_choice != 'n' {
            println!("How many portions: ");
            io::stdin().read_line(&mut input2).expect("Failed to understand");
            quantity = input2.trim().parse().expect("Not an integer value");

        
            println!("{}; quantity: {}; unit price: {}", menu.get(&users_choice).unwrap(), quantity, price.get(&users_choice).unwrap() * quantity);
            println!("Would that be all");
            //unwrap allows you to quickly access values returned by various functions such as a function returning a Result objected.
            
            total_price = total_price + (price.get(&users_choice).unwrap() * quantity) as f32;
        }

    }

    if total_price > 10000.0 {
        println!("N{}", total_price);
        total_price = total_price * (0.95);
        println!("Congratulations you get a discount");
        println!("Total price: N{}", total_price);
        
    }else{
        
        println!("Total price: N{}", total_price);
    }

    println!("\t\tThanks for eating at DOUBLE Gs Dinner\n\t\tCan't wait to see you again");
}



fn main() {
    display();
    get_customer_order();
    
}
