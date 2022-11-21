fn main() {
    let fullname = "Godsgift David Ifeany";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";

    let mut school = "School of Science".to_string();
    // push string
    school.push_str(" and Technology");

    println!("My name is: {}", fullname );
    //check lenght 
    println!("The lenght of my fullname is {}",fullname.len() );
    println!("I am a student of {} Department",department );
    println!("im part of {}",school);
    println!("and im in {}", uni);
}
