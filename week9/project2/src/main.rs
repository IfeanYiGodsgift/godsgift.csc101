use std::io::Write;

fn display(){
    let v2 = vec!["Oluchi Mordi\t","Adams Aliyu\t\t","Shania Bolade\t","Adekunle Gold\t","Blanca Edomoh\t"];
    let v3 = vec!["ACC10211111\t","ECO10110101\t","CSC10328828\t","EEE11020202\t","MEE10202001\t",""];
    let v4 = vec!["\t  Accounting\t  300\n","\t  Economics\t\t  100\n","\t  Computer\t\t  200\n","\t  Electrical\t  200\n","\t  Mechanical\t  100\n",""];
    let mut file = std::fs::File::create("Pau sims.txt").expect("create failed");
    file.write_all("Student Name   \tMatric.Number     Department      Level\n".as_bytes()).expect("write failed");

    for i in 0..v2.len(){
        file.write_all(v2[i].as_bytes()).expect("write failed");
        file.write_all(v3[i].as_bytes()).expect("write failed");
        file.write_all(v4[i].as_bytes()).expect("write failed");
    }
}


fn main() {
   display() ;
}
