fn main() {
<<<<<<< HEAD
    println!("Hello, world!");
}
=======
	let b:(i32,bool,f64) = (30,true,4.9);
	call(b);
}

fn call(x:(i32,bool,f64)) {
	println!("Inside print method");
	let (age,is_male,cgpa) = x;
	println!("Age is {}, isMale? {}, cgpa is {}",age,is_male,cgpa);
}
>>>>>>> 0273788f0e7bc39bc0c7c00a6890d24123e56c48
