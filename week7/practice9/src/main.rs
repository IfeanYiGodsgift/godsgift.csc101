fn main() {
//Tuples can be brokn down to smaller variable

let b:(i32,bool,f64) = (30,true,4.9);
pass(b);
}

fn pass(x:(i32,bool,f64)){

    println!("Inside print method");
    //assings a tuple to distinct variables per index
    let (age,is_male,cgpa) = x;
    println!("Age is {},\nIs male? {},\nCgpa is {}",age,is_male,cgpa );

}