fn main() {
    println!("passing tuple by value:\nby Godsgift Ifeanyi\n");

    let b:(i32,bool,f64)= (110,true,12.3);
    Pass(b);
}

fn Pass(x:(i32,bool,f64)) {

    println!("Inside print method");
    println!("{:?}",x);
}
