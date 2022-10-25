fn main() {
   // T:TOshiba, M:MAC, H:HP (ABREVEIATING TO KEEP CONSISTENCY), D:DELL, A:ACER
   		let t1 = 2;
   		let m1 = 1;
   		let h1 = 3;
   		let d1 = 3;
   		let a1 = 1;

   		let t2 = 450_000;
   		let m2 = 1_500_000;
   		let h2 = 750_000;
   		let d2 = 2_850_000;
   		let a2 = 250_000;

   		let toshiba = t1*t2;
   		let mac = m1*m2;
   		let hp = h1*h2;
   		let dell = d1*d2;
   		let acer = a1*a2;

   	/* godsgift did this to elaborate on how he is solving
   	S is for sum and Avg is obviously average 
   	so something happened and turns out i have to make every thing lowwer case so rather ill just right the full names */
   		let s = toshiba+mac+hp+dell+acer;
   		println!("Sum of sales record {}", s);
   		let avg = s/5;
   		println!("Average of sales record is {}", avg);
}