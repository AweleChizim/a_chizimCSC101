fn main()  {
	// q is for Quantity
	let q1 = 2;
	let q2 = 1;
	let q3 = 3;
	let q4 = 3;
	let q5 = 1;

	//a is for Amount
	let a1 = 450000;
	let a2 = 1500000;
	let a3 = 750000;
	let a4 = 2850000;
	let a5 = 250000;

	/*t is for Toshiba, m is for Mac, 
	hp is for HP, d is Dell,
	a is for Acer*/
	let t = q1 * a1;
	let m = q2 * a2;
	let hp = q3 * a3;
	let d = q4 * a4;
	let a = q5 * a5;

	/* sum = Sum of sales recorded
	s = sum of Quantity
	average = Average sales recorded */
	let sum = t + m + hp + d + a;
	println!("Sum of sales recorded is {}", sum );
	let s = q1 + q2 + q3 + q4 + q5;
	let average = sum / s;
	println!("Average sales recorded is {}", average );



}