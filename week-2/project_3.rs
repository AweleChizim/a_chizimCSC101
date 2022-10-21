fn main()  {
	let p:f64 = 210000.00;
	let r:f64 = 5.00;
	let n:f64 = 3.00;
	let c:f64 = 1.00 - (r / 100.00);
	let c = f64 ::powf(c,n);

	//d is for depreciation
	let a = p * c;
	let d = p - a;
	println!("The value of the TV after 3 years is {}", d );

}