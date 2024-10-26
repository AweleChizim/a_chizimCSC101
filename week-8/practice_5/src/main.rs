use std::io;

fn main() {
	let mut city : Vec<String> = Vec::new();
	println!("The City vector has element {}", city.len());

	let mut input1 = String::new();
	println!("Enter an index value btw (0 - 7)");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let index:i32 = input1.trim().parse().expect("Invalid input");

    for count in 0..index {
    	let mut input2 = String::new();
<<<<<<< HEAD
		println!("Enter City {}", index+1);
	    io::stdin().read_line(&mut input2).expect("Failed to read line");
	    let new_city:i32 = input2.trim().parse().expect("Invalid input");
=======
		println!("Enter City Name");
	    io::stdin().read_line(&mut input2).expect("Failed to read line");
	    let new_city:String = input2.trim().parse().expect("Invalid input");
>>>>>>> 0273788f0e7bc39bc0c7c00a6890d24123e56c48
	    city.push(new_city);
    }
    println!("Your preferred cities are:\n");
    let mut count=1;
    for i in city
    {
<<<<<<< HEAD
    	print!("{} {}", count, i);
=======
    	print!("{}.{} ", count, i);
>>>>>>> 0273788f0e7bc39bc0c7c00a6890d24123e56c48
    	count+=1;
    }
}
