use std::io;

//property of GODSGIFT DAVID IFEANYI
fn main() {
    let mut city:Vec<String> = Vec::new();

    println!("The city vector has a element {}",city.len() );
    //push new elements in city

    let mut input1 = String::new();
    println!("How many cities do you want to enter");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num:i32 = input1.trim().parse().expect("Invalid input");
    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter city {}", count+1);
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_city:String = input2.trim().parse().expect("Invalid input");
        city.push(new_city);

    }
    print!("Your preferred cities are: \n");
    //you can always remove the count=1 it wont spoil it
    // but it wont bulett point so the choice is yours
    let mut count=1;
    for i in city {
        println!("{} {}",count,i);
        count+=1;
    } 

}
