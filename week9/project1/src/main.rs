use std::io::Write;

fn display(){
    let v2 = vec!["33 Export\t","Desperados\t","Goldberg\t","Gulder\t\t\t\t\t","Heineken\t\n","Star\t"];
    let v3 = vec!["Legend\t","Turbo king\t","Williams\t","","",""];
    let v4 = vec!["\tMaltina\t\n","Amstel Malta\t\n","Malta Gold\t\n","Fayrouz\t\n","",""];

    let mut file = std::fs::File::create("NBL.txt").expect("create failed");
    file.write_all("Lager   \tStout       \tNon-Alcoholic\n".as_bytes()).expect("write failed");

    for i in 0..v2.len(){
        file.write_all(v2[i].as_bytes()).expect("write failed");
        file.write_all(v3[i].as_bytes()).expect("write failed");
        file.write_all(v4[i].as_bytes()).expect("write failed");
    }
}


fn main() {
   display() ;
}
