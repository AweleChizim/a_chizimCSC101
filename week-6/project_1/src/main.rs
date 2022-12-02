//Rust program to select a formula, read inputs and print result. 
use std::io;

fn main() {

	let s_n:[i32;5] = [1, 2, 3, 4, 5];
	let formula = ["Area of Trapezium formula", "Area of Rhombus formula", "Area of Parallelogram formula", "Area of Cube formula", "Volume of Cylinder formula"];
	println!("SELECT A FORMULA:-");
	for index in 0..formula.len(){
		println!("Click {} for {}", s_n[index], formula[index]);
	    }

    let mut formula_selected = String::new();
    io::stdin().read_line(&mut formula_selected).expect("Failed to read input");
    let formula_selected:i32 = formula_selected.trim().parse().expect("Not a valid integer");

    // calling a_o_t function
    if formula_selected == 1 {
    	a_o_t();
	}

	else if formula_selected == 2 {
		a_o_r();
	}

	else if formula_selected == 3 {
		a_o_p();
	}

	else if formula_selected == 4 {
		a_o_c();
	}

	else if formula_selected == 5 {
		v_o_c();
	}

	else {
		println!("Invalid Input!\nPlease try again...");
	}
} 


// a_o_t = Area of Trapezium 
fn a_o_t() {
	println!("Area of Trapezium = (height/2)*(base1 + base2)");
	let mut height = String::new();
	println!("Input height:-");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height:f64 = height.trim().parse().expect("Not a valid integer");

    let mut base_1 = String::new();
	println!("Input base 1:-");
    io::stdin().read_line(&mut base_1).expect("Failed to read input");
    let base_1:f64 = base_1.trim().parse().expect("Not a valid integer");

    let mut base_2 = String::new();
	println!("Input base 2:-");
    io::stdin().read_line(&mut base_2).expect("Failed to read input");
    let base_2:f64 = base_2.trim().parse().expect("Not a valid integer");

    let x:f64 = height/2.0;
    let y:f64 = base_1 + base_2;
    let z:f64 = x * y;
    println!("Calculating...\nArea of the Trapezium is {}cm^2", z);
}

// a_o_r = Area of Rhombus
fn a_o_r() {
	println!("Area of Rhombus = 0.5 * diagonal1 x diagonal2");
	let mut diagonal_1 = String::new();
	println!("Input diagonal 1:-");
    io::stdin().read_line(&mut diagonal_1).expect("Failed to read input");
    let diagonal_1:f64 = diagonal_1.trim().parse().expect("Not a valid integer");

    let mut diagonal_2 = String::new();
	println!("Input diagonal 2:-");
    io::stdin().read_line(&mut diagonal_2).expect("Failed to read input");
    let diagonal_2:f64 = diagonal_2.trim().parse().expect("Not a valid integer");

	let z:f64 = 0.5 * diagonal_1 * diagonal_2;
    println!("Calculating...\nArea of the Rhombus is {}cm^2", z);
}

// a_o_p = Area of Parallelogram
fn a_o_p() {
	println!("Area of Parallelogram = base x altitude");
	let mut base = String::new();
	println!("Input base:-");
    io::stdin().read_line(&mut base).expect("Failed to read input");
    let base:f64 = base.trim().parse().expect("Not a valid integer");

    let mut altitude = String::new();
	println!("Input altitude:-");
    io::stdin().read_line(&mut altitude).expect("Failed to read input");
    let altitude:f64 = altitude.trim().parse().expect("Not a valid integer");

	let z:f64 = base * altitude;
    println!("Calculating...\nArea of the Parallelogram is {}cm^2", z);
}

// a_o_c = Area of Cube 
fn a_o_c() {
	let mut length = String::new();
	println!("Input length of any side (all sides are equal):-");
    io::stdin().read_line(&mut length).expect("Failed to read input");
    let length:f64 = length.trim().parse().expect("Not a valid integer");

    let y:f64 = f64::powi(length, 2);
	let z:f64 = 6.0 * y;
    println!("Calculating...\nArea of the Cube is {}cm^2", z);
}

// v_o_c = Volume of Cylinder
fn v_o_c() {
	println!("Volume of Cylinder = pi x radius^2 x height");
	let mut radius = String::new();
	println!("Input radius:-");
    io::stdin().read_line(&mut radius).expect("Failed to read input");
    let radius:f64 = radius.trim().parse().expect("Not a valid integer");

    let mut height = String::new();
	println!("Input height:-");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height:f64 = height.trim().parse().expect("Not a valid integer");

    let x:f64 = 22.0 / 7.0;
    let y:f64 = f64::powi(radius, 2);
	let z:f64 = x * y * height;
    println!("Calculating...\nVolume of the Cylinder is {}cm^3", z);
}