/* Pass by value 
    Practice 5
    Property of GODSGIFT iFEANYI
*/

fn mutate_num_to_zero(mut param_num: i32){
  param_num = param_num*0;
  println!("Param_num value is : {}",param_num);
}

fn main() {
    let num:i32 = 5;
    mutate_num_to_zero(num);
    println!("The value of no is: {}",num);
}
