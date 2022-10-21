fn main()  {
	let P:f64 = 520000000.0;
	let R:f64 = 10.0;
	let N:f64 = 5.0

	//compound interest
	let A = P[1 + (R / 100)]N
	println!("Amount is {}", A );
	let CI = A - P
	println!("Compound Interest is {}", CI );

}