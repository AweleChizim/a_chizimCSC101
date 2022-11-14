//Rust program to calculate the area of a trinagle for a given base and height 

use std::io;

fn main() 
{
   let mut base = String::new();
   let mut height = String::new();
   
   println!("Enter base:");
   io::stdin().read_line(&mut base).expect("Failed to read input");
   let base:f32 = base.trim().parse().expect("Not a valid integer");

   println!("Enter height:");
   io::stdin().read_line(&mut height).expect("Failed to read input");
   let height:f32 = height.trim().parse().expect("Not a valid integer");

   if base > 0.0 {
   		let area:f32 = (base * height) / 2.0;
   		println!("Area of a triangle: {}", area); 
   }
}
