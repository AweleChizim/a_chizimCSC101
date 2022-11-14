// Rust program to determine the annual incentive of an employee

use std::io;

fn main() 
{

	// Input experience
	let mut experience = String::new();
    println!("\nEnter 1 for experienced and 2 for inexperienced");
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience:i32 = experience.trim().parse().expect("Not a valid integer");


    // Input age
    let mut age = String::new();
    println!("Enter age:");
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Not a valid integer");

    if experience == 2
    {
    	let mut ai:i32 = 100000 * 12;
    	println!("Employee's annual incentive is {}.", ai);
    }
    else if (age >= 40) && (experience == 1)
    {
    	let ai:i32 = 1500000 * 12;
    	println!("Employee's annual incentive is {}.", ai);
    }
    else if (age >= 30) && (age < 40) && (experience == 1)
    {
    	let ai:i32 = 1480000 * 12;
    	println!("Employee's annual incentive is {}.", ai);
    }
    else if (age < 28) && (experience == 1)
    {
    	let ai:i32 = 1300000* 12;
    	println!("Employee's annual incentive is {}.", ai);
    }
    else
    {
    	println!("Invalid Input");
    }
}
