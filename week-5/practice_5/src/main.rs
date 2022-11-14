// Rust code to read a persons height and print if the person is tall, dwarf or average height

use std::io;

fn main() 
{	
	let mut input = String::new();

    println!("\n Enter you height (in cm):");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.00 && height <= 170.00
    {
    	println!("You are a person of average height");
    }
    else if height > 170.00 && height <= 195.00
    {
    	println!("You are tall");
    }
    else if height < 150.0 && height > 100.0
    {
    	println!("You are a dwarf");
    }
    else 
    {
    	println!("Abnormal height");
    }
}
