fn main() {
	let p:f64 = 510000.0;
	let r:f64 = 0.05;
	let t:f64 = 3.0;

	//Depreciation
	let v = p * ( 1.0 - r) * t;
	println!("Depriciation is {}", v);

}