
/*declaring and initializing arrays
    PROPERTY OF GODSGIFT IFEANYI
    practice 7
*/
fn main(){
  //explicit integer datatype
  let arr1:[i32;4] = [10,20,30,40];
  println!("\n Array with data type");
  println!("array is {:?}",arr1);
  println!("array size is : {}",arr1.len());

    //implicit float datatype
  let arr2 = [10.4,20.7,30.4,40.7];
  println!("\n Array without data type");
  println!("array is {:?}",arr2);
  println!("array size is : {}",arr2.len());

    //array whit  default values that creates and 
  // initializes all its elements with a default value of -1
  let arr3:[i32;8] = [-1;8];
  println!("\n Array with default values");
  println!("array is {:?}",arr3);
  println!("array size is : {}",arr3.len());
}

