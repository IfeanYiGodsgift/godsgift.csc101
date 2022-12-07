fn main() {
    //Property of Godsgift david ifeanyi
    //below tuple contains a dataype :()
    let tuple:(&str,f32,u8) = ("Rust",3.15,100);
    println!("Tuple contents = {:?}",tuple);

    //below tuple contoans no datype
    let nodtp_tuple = ("Rust","Fun",100);
    println!("tuple contents = {:?}", nodtp_tuple);

    // accessing elemints at index 0 using: [tuplename].[index]
    println!("Value at Index 0: {}",tuple.0);

    println!("Value at Index 1: {}",tuple.1);

    println!("Value at Index 2: {}",tuple.2);

    println!("Value at Index 0.0: {}",nodtp_tuple.0);

    println!("Value at Index 1.0: {}",nodtp_tuple.1);

    println!("Value at Index 2.0: {}",nodtp_tuple.2);

}
