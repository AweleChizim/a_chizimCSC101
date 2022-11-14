// Rust program to find the roots of a quadratic equation.

use std::io;

fn main() {
	// Input a
    println!("\nEnter the value of a.");
    let mut a = String::new();
    	io::stdin().read_line(&mut a).expect("Failed to read input");
    let a:f32 = a.trim().parse().expect("Input not an integer");

    // Input b
    println!("\nEnter the value of b.");
    let mut b = String::new();
    	io::stdin().read_line(&mut b).expect("Failed to read input");
    let b:f32 = b.trim().parse().expect("Input not an integer");

    // Input c
    println!("\nEnter the value of c.");
    let mut c = String::new();
    	io::stdin().read_line(&mut c).expect("Failed to read input");
    let c:f32 = c.trim().parse().expect("Input not an integer");

    let d = f32 ::powf(b,2.0);
    let e = d - (4.0 * a * c);
    let f = e.sqrt();
    let root1 = (b + f) / (2.0 * a);
    let root2 = (b - f) / (2.0 * a);

    println!("The roots of the quadratic equation are {} and {}.", root1, root2);
}
