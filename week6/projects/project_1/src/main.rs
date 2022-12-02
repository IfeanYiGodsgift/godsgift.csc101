use std::io;
use colored::Colorize;

fn display(){
    println!("\n\t\tGodsgift area of shape calculator\n\tArea of a Trapezium\t 1\n\tArea of a Rhombus\t 2\n\tArea of a Parallelogram  3\n\tArea of a Cube\t\t 4\n\tVolume of a Cylinder\t 5");
}

fn area_trap(a:f32,b:f32,c:f32) {

    let area = c/2.0*(a + b);
    println!("Area of a Trapezium is {}", area);
}

fn area_rhomb(d1:f32, d2:f32){
    let area = 0.5 * d1 * d2;
    println!("Area of a Rhombus is {}", area);
}

fn area_parallelogram(b:f32, a:f32){

    let area = b * a ;
    println!("Area of a Parallelogram is {}", area);
}   
fn area_cube(l:f32){
    let pow = f32::powi(l,2);
    let area = 6.0 * pow;
    println!("Area of a Cube is : {}", area);
}

fn vol_cylinder(r:f32, h:f32){
    let pi:f32 = 22.0 / 7.0;
    let rad = f32::powi(r,2);
    let vol = pi * rad * h ;
    println!("Volume of a cylinder is {}", vol);
}

fn calc(){
    let mut game :bool = true;

let mut user_choice:i32 = 9;
    
    
    while game{
    let mut input = String::new();
    println!("what do u want to do :");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let user_choice:i32 = input.trim().parse().expect("Imvalid input");
 
   
    if user_choice == 1{

        println!("You have chosen to find Area of a Trapezium");

        let mut input1 = String::new();
    println!("Enter input for parameter Base1:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter input for parameter Base2:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Invalid input");

    let mut input3 = String::new();
    println!("Enter input for parameter Height:");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = input3.trim().parse().expect("Invalid input");

    area_trap(a, b, c)
    }

    else if user_choice == 2{

        let mut input4 = String::new();
    println!("Enter input for Diagonal 1:");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let d1:f32 = input4.trim().parse().expect("Invalid input");

    let mut input5 = String::new();
    println!("Enter input for parameter 2:");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let d2:f32 = input5.trim().parse().expect("Invalid input");

    area_rhomb(d1,d2)


    }
    else if user_choice == 3{

        println!("You have choosen to find area of a Parallelogram");

        let mut input6 = String::new();
        println!("Enter input for Base :");
        io::stdin().read_line(&mut input6).expect("Failed to read input");
        let b:f32 = input6.trim().parse().expect("Invalid input");

        let mut input7 = String::new();
        println!("Enter input for altitude :");
        io::stdin().read_line(&mut input7).expect("Failed to read input");
        let a:f32 = input7.trim().parse().expect("Invalid input");
        
        area_parallelogram(b,a) 
    }
    else if user_choice == 4{
        let mut input8 = String::new();
        println!("Enter the length of the sides :");
        io::stdin().read_line(&mut input8).expect("Failed to read input");
        let l:f32 = input8.trim().parse().expect("Invalid input");

        area_cube(l)
    }
    else if user_choice == 5{
        let mut input9 = String::new();
        println!("Enter the value of radius :");
        io::stdin().read_line(&mut input9).expect("Failed to read input");
        let r:f32 = input9.trim().parse().expect("Invalid input");

        let mut input10 = String::new();
        println!("Enter the value of Height :");
        io::stdin().read_line(&mut input10).expect("Failed to read input");
        let h:f32 = input10.trim().parse().expect("Invalid input");

        vol_cylinder(r,h)

    }
    else if user_choice == 0{
        println!("Thank you for using my calculator\nNew updates coming soon");
        game = false;
    }
    else {
        println!("{} ❌🛑⛔", format!("Invalid submission").red() );
    }
    }
    println!("BYE");


}

fn main() {

    display();

    calc();
    
}


    
