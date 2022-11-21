fn main() {
    let fullname = " Pan-Atlantic University ";
    println!();
    println!("Name: {}", fullname);
    println!();
    println!("Before trim");
    println!("Lenght is {}", fullname.len());
    println!();
    println!("After trim");
    println!("Lenght is {}", fullname.trim().len());
}
