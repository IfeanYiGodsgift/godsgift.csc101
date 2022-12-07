fn main() {
   // an array of numbers
   let numbers = [1,2,3,4,5];
   println!("Original array {:?}",numbers);

   //creating a slice of 2nd and 3rd element
   let slice1 = &numbers[1..3];
   println!("2nd and 3rd elements sliced = {:?}",slice1);

   //omit the start index
   let slice2 = &numbers[..3];
   //slice shall start from 0 index to 3 index(exclusive)
   // anything past 2nd index is not shown
   println!("Index 0 to index 2 sliced = {:?}",slice2);

   //omit the end index
   let slice3 = &numbers[1..];
   //slice starts from index 1 and ends at last index
   println!("Index 1 to index 4 = {:?}",slice3);

   // omit both ends to reference an array
   let slice4 = &numbers[..];
   println!("Index 0 to 4 = {:?}",slice4);
}
