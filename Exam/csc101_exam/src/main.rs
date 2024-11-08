use std::io;
use std::io::Read;
use std::io::Write;



fn main() {
    println!("Welcome User");
    
    let mut user = String::new();
    println!("Please input who user is. \nFor Physician, type 1; \nFor Patient, type 2");
    io::stdin().read_line(&mut user).expect("Failed to read input");
    let user:i32 = user.trim().parse().expect("Not a valid integer");

    let allergy_id_a = vec![3, 1, 2, 5, 2, 4, 1, 3, 5, 1];
    let specialization_list = vec!["Surgery", "Pediatrics", "Orthopedic"];
    let email_list = vec!["a-seidu@gmail.com", "m-gbenga@gmail.com", "g-audu@gmail.com"];

    if user == 1 {
    	let mut email = String::new();
    	println!("Please input Your Email (all lowercase):");
    	io::stdin().read_line(&mut email).expect("Failed to read input");
    	
    	let mut specialization = String::new();
    	println!("Please input Your Specialization (all lowercase):");
    	io::stdin().read_line(&mut specialization).expect("Failed to read input");
    		
    	for i in 0..email_list.len() {

    		if specialization == specialization_list[i] {
    			hospital_db()
    		}
    		
    		else {
    			println!("Invalid User");
   			}
   		}
    }
    
    if user == 2 {
    	let mut email = String::new();
    	println!("Please input Your Email (all lowercase):");
    	io::stdin().read_line(&mut email).expect("Failed to read input");
    	
    	if (email == "a-simon@gmail.com") || (email == "f-tina@gmail.com") || (email == "d-valerie@gmail.com") || (email == "s-samuel@gmail.com") || (email == "o-feji@gmail.com") || (email == "m-kabir@gmail.com") || (email == "a-jane@gmail.com") || (email == "m-ali@gmail.com") || (email == "o-chisom@gmail.com") || (email == "e-agatha@gmail.com") {
    		for i in 0..allergy_id_a.len() {
    			if allergy_id_a[i] < 3 {
    				allergy_less_3();
    			}
    			if allergy_id_a[i] > 3 {
    				allergy_greater_3();
    			}
    			else {
    				println!("No Access Permission");
    			}
    		}
    	}
    }
}

fn hospital_db() {
	let mut Physician = std::fs::File::open("achizim_db.sql").unwrap();
	let mut database = String::new();
	Physician.read_to_string(&mut database).unwrap();
   	print!("{}", database);
}

fn allergy_less_3() {
	let pa_id = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	let pa_id2 = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
	let l_name = vec!["Agada", "Fagbemi", "Dalegi", "Salami", "Oghenero", "Mustapha", "Alokwe", "Makman", "Okonkwo", "Eze"];
	let f_name = vec!["Simon", "Tina", "Valerie", "Samuel", "Feji", "Kabir", "Jane", "Ali", "Chisom", "Agatha"];
	let e_mail = vec!["a-simon@gmail.com", "f-tina@gmail.com", "d-valerie@gmail.com", "s-samuel@gmail.com", "o-feji@gmail.com", "m-kabir@gmail.com", "a-jane@gmail.com", "m-ali@gmail.com", "o-chisom@gmail.com", "e-agatha@gmail.com"];
	let dob = vec!["12/3/1992", "3/12/1989", "11/1/1993", "21/7/1998", "10/6/1991", "13/5/1990", "28/11/1988", "1/12/2000", "15/11/1999", "22/4/1996"];
	let ph_id = vec!["2", "3", "1", "2", "3", "2", "3", "1", "1", "2"];
	let allergy_id_a1 = vec![3, 1, 2, 5, 2, 4, 1, 3, 5, 1];
	let allergy_id_a = vec!["3", "1", "2", "5", "2", "4", "1", "3", "5", "1"];
	let ph_name = vec!["Seidu Ahmed", "Gbenga Mildred", "Audu Gloria", "Seidu Ahmed", "Gbenga Mildred", "Seidu Ahmed", "Gbenga Mildred", "Audu Gloria", "Audu Gloria", "Seidu Ahmed"];
	let ph_spe = vec!["Surgery", "Pediatrics", "Orthopedic", "Surgery", "Pediatrics", "Surgery", "Pediatrics", "Orthopedic", "Orthopedic", "Pediatrics"];
	let ph_email = vec!["a-seidu@gmail.com", "m-gbenga@gmail.com", "g-audu@gmail.com","a-seidu@gmail.com", "m-gbenga@gmail.com", "a-seidu@gmail.com", "m-gbenga@gmail.com", "g-audu@gmail.com", "g-audu@gmail.com", "m-gbenga@gmail.com"];
	
	for i in 0..pa_id.len()
	{
		if allergy_id_a1[i] < 3 {
			let mut file = std::fs::File::create(pa_id2[i]).expect("create failed");
				file.write_all("Patient ID:- ".as_bytes()).expect("write failed");
				file.write_all(pa_id2[i].as_bytes()).expect("write failed");
				file.write_all("\n".as_bytes()).expect("write failed");
				file.write_all("Last Name:- ".as_bytes()).expect("write failed");
				file.write_all(l_name[i].as_bytes()).expect("write failed");
				file.write_all("\n".as_bytes()).expect("write failed");
				file.write_all("First Name:- ".as_bytes()).expect("write failed");
				file.write_all(f_name[i].as_bytes()).expect("write failed");
				file.write_all("Email:- ".as_bytes()).expect("write failed");
				file.write_all(e_mail[i].as_bytes()).expect("write failed");
				file.write_all("\n".as_bytes()).expect("write failed");
				file.write_all("Date of Birth:- ".as_bytes()).expect("write failed");
				file.write_all(dob[i].as_bytes()).expect("write failed");
				file.write_all("\n".as_bytes()).expect("write failed");
				file.write_all("Physicians ID:- ".as_bytes()).expect("write failed");
				file.write_all(ph_id[i].as_bytes()).expect("write failed");
				file.write_all("Physicians Name:- ".as_bytes()).expect("write failed");
				file.write_all(ph_name[i].as_bytes()).expect("write failed");
				file.write_all("Physicians Email:- ".as_bytes()).expect("write failed");
				file.write_all(ph_email[i].as_bytes()).expect("write failed");
				file.write_all("\n".as_bytes()).expect("write failed");
				file.write_all("Physicians Specialization:- ".as_bytes()).expect("write failed");
				file.write_all(ph_spe[i].as_bytes()).expect("write failed");
				file.write_all("\n".as_bytes()).expect("write failed");
				file.write_all("Allergy ID:- ".as_bytes()).expect("write failed");
				file.write_all(allergy_id_a[i].as_bytes()).expect("write failed");
				file.write_all("\n".as_bytes()).expect("write failed");
		}
	}
}

fn allergy_greater_3() {
	let pa_id = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	let pa_id2 = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
	let l_name = vec!["Agada", "Fagbemi", "Dalegi", "Salami", "Oghenero", "Mustapha", "Alokwe", "Makman", "Okonkwo", "Eze"];
	let f_name = vec!["Simon", "Tina", "Valerie", "Samuel", "Feji", "Kabir", "Jane", "Ali", "Chisom", "Agatha"];
	let e_mail = vec!["a-simon@gmail.com", "f-tina@gmail.com", "d-valerie@gmail.com", "s-samuel@gmail.com", "o-feji@gmail.com", "m-kabir@gmail.com", "a-jane@gmail.com", "m-ali@gmail.com", "o-chisom@gmail.com", "e-agatha@gmail.com"];
	let dob = vec!["12/3/1992", "3/12/1989", "11/1/1993", "21/7/1998", "10/6/1991", "13/5/1990", "28/11/1988", "1/12/2000", "15/11/1999", "22/4/1996"];
	let ph_id = vec!["2", "3", "1", "2", "3", "2", "3", "1", "1", "2"];
	let allergy_id_a1 = vec![3, 1, 2, 5, 2, 4, 1, 3, 5, 1];
	let allergy_id_a = vec!["3", "1", "2", "5", "2", "4", "1", "3", "5", "1"];
	let al_name = vec!["Diabetes", "Low sugar", "Low cholesterol", "Fish", "Low cholesterol", "Anaphylaxis", "Low sugar", "Diabetes", "Fish", "Low sugar"];
	let symptoms = vec!["Surgery", "Pediatrics", "Orthopedic", "Surgery", "Pediatrics", "Surgery", "Pediatrics", "Orthopedic", "Orthopedic", "Pediatrics"];
	

	for i in 0..pa_id.len()
	{
		if allergy_id_a1[i] > 3 {
			let mut file = std::fs::File::create(pa_id2[i]).expect("create failed");
				file.write_all("Patient ID:- ".as_bytes()).expect("write failed");
				file.write_all(pa_id2[i].as_bytes()).expect("write failed");
				file.write_all("\n".as_bytes()).expect("write failed");
				file.write_all("Last Name:- ".as_bytes()).expect("write failed");
				file.write_all(l_name[i].as_bytes()).expect("write failed");
				file.write_all("\n".as_bytes()).expect("write failed");
				file.write_all("First Name:- ".as_bytes()).expect("write failed");
				file.write_all(f_name[i].as_bytes()).expect("write failed");
				file.write_all("Email:- ".as_bytes()).expect("write failed");
				file.write_all(e_mail[i].as_bytes()).expect("write failed");
				file.write_all("\n".as_bytes()).expect("write failed");
				file.write_all("Date of Birth:- ".as_bytes()).expect("write failed");
				file.write_all(dob[i].as_bytes()).expect("write failed");
				file.write_all("\n".as_bytes()).expect("write failed");
				file.write_all("Physicians ID:- ".as_bytes()).expect("write failed");
				file.write_all(ph_id[i].as_bytes()).expect("write failed");
				file.write_all("\n".as_bytes()).expect("write failed");
				file.write_all("Allergy ID:- ".as_bytes()).expect("write failed");
				file.write_all(allergy_id_a[i].as_bytes()).expect("write failed");
				file.write_all("Name of allergy:- ".as_bytes()).expect("write failed");
				file.write_all(al_name[i].as_bytes()).expect("write failed");
				file.write_all("Allergy Symptoms:- ".as_bytes()).expect("write failed");
				file.write_all(symptoms[i].as_bytes()).expect("write failed");
		}
	}
}
