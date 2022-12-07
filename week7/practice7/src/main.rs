fn main() {

// Normaly in rust tuples arent mutable meaning you cant really change them 
// However if you add the keyword (mut) before assigning the tuple then it 
// becomes a mutable tuple

let mut tuple1 = ("Everest",8848,"Fishtail",6993);
println!("Original tuple = {:?}",tuple1);

//changing the 3rd and 4th elements of an already existing tuple
// 3rd element is index 2 and 4th is index 3
tuple1.2 = "Lhotse";
tuple1.3 = 85116; 

println!("Changed tuple = {:?}", tuple1);

}
