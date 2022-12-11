use std::io::Write;
use std::io::Read;

fn main() {
	let student_name =  vec!["Oluchi Mordi     ", "Adams Aliyu      ", "Shanla Bolade    ", "Adekunle Gold    ", "Blanca Edemoh    "];
	let matric_number = vec!["ACC10211111      ", "ECO10110101      ", "CSC10328828      ", "EEE11020202      ", "MEE10202001      "];
	let department = vec!["Accounting            ", "Economics             ", "Computer Science      ", "Electrical Engineering", "Mechanical Engineering"];
	let level = vec!["300              ", "100              ", "200              ", "200              ", "100              "]; 
    let s_n = vec![" 1  ", " 2  ", " 3  ", " 4  ", " 5  "]; 
   
	let mut sims_file = std::fs::File::create("PAU_SIMS.txt").expect("create failed");
	sims_file.write_all("S/N  | Student Name      |   Matric. Number  | Department             | Level\n".as_bytes()).expect("write failed");
	sims_file.write_all("_____________________________________________________________________________\n".as_bytes()).expect("write failed");

	for i in 0..department.len(){
		sims_file.write_all(s_n[i].as_bytes()).expect("write failed");
		sims_file.write_all(" | ".as_bytes()).expect("write failed");
		sims_file.write_all(student_name[i].as_bytes()).expect("write failed");
		sims_file.write_all(" | ".as_bytes()).expect("write failed");
		sims_file.write_all(matric_number[i].as_bytes()).expect("write failed");
    	sims_file.write_all(" | ".as_bytes()).expect("write failed");
    	sims_file.write_all(department[i].as_bytes()).expect("write failed");
    	sims_file.write_all(" | ".as_bytes()).expect("write failed");
    	sims_file.write_all(level[i].as_bytes()).expect("write failed");
    	sims_file.write_all("\n".as_bytes()).expect("write failed");
	}

	let mut sims = std::fs::File::open("PAU_SIMS.txt").unwrap();
	let mut sims_content = String::new();
	sims.read_to_string(&mut sims_content).unwrap();
    print!("{}", sims_content);

}
