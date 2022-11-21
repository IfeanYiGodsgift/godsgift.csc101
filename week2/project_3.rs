fn main() {
	// so to find depreciation im just going to use the ci formulae but the r is negative
	// a godsgift original project
    // i would need to be informed on how to add exponents
    let p:f64 = 210_000.0;
    let n:f64 = 3.0;
    let r:f64 = 5.0;

    let d = p*(1.0-(r/100.0))*n;
    println!("Depreciation of Ms.Akuudo's tv is {}",d);
}