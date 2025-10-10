fn main() {
	let p:f64 = 520000000.0;
	let r:f64 = 0.1;
	let t:f64 = 5.0;

	// compound intrest
	let a = p * ( 1.0 + (r / 100.0)) * t;
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Compound Intrest is {}", ci);

}