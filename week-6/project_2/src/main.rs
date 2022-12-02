// Rust code for user to input the data of their sibling(s) and have the data printed out. 
use std::io;

fn main() {
    println!("Good day ma/sir");
    let mut no_of_siblings = String::new();
    println!("How many sibling do you have?");
    io::stdin().read_line(&mut no_of_siblings).expect("Failed to read input");
    let no_of_siblings:i32 = no_of_siblings.trim().parse().expect("Not a valid integer");
    let b = no_of_siblings + 1;

    let mut data = String::new();
    
    if no_of_siblings == 0 {
    	println!("Client has no siblings");
    }

    if no_of_siblings > 0 {
	    for i in 1..b {
	   		// data1(i);
	   		let mut data_details = String::new();

		let mut sibling_name = String::new();
		println!("Input name of sibling {}:-", i);
	 	io::stdin().read_line(&mut sibling_name).expect("Failed to read input");
	  	
	  	let mut sibling_age = String::new();
		println!("Input sibling age:-");
	 	io::stdin().read_line(&mut sibling_age).expect("Failed to read input");
	  	let sibling_age:i32 = sibling_age.trim().parse().expect("Not a valid input");

	  	let mut sibling_age_2 = String::new();
	  	let mut sibling_name_2 = String::new();
	  	let mut g = sibling_name.to_string();
	  	let mut f = sibling_age.to_string();
	  	sibling_name_2 = "Name of Sibling:".to_string() + &g;
	  	sibling_age_2  = "Age:".to_string() + &f;


	  	if sibling_age >= 18 {
	  		let mut marital_status = String::new();
	  		let mut relationship = String::new();
		    println!("Enter 1 if married and 2 if single");
		    io::stdin().read_line(&mut marital_status).expect("Failed to read input");
		    let marital_status:i32 = marital_status.trim().parse().expect("Not a valid integer");
		    
		    if marital_status == 1 {
		    	relationship = "Marital Status: Married".to_string();
		    	let mut city_of_residence = String::new();
		    	let mut city_of_residence_2 = String::new();
				println!("What city does the family live in?");
			 	io::stdin().read_line(&mut city_of_residence).expect("Failed to read input");

			  	
			  	let mut no_of_children = String::new();
			  	let mut no_of_children_2 = String::new();
				println!("How many children does he/she have?");
			 	io::stdin().read_line(&mut no_of_children).expect("Failed to read input");
			  	let no_of_children:i32 = no_of_children.trim().parse().expect("Not a valid input");

			  	city_of_residence_2 = "City of Residence:".to_string() + &city_of_residence;
			  	no_of_children_2 = "No of Children:".to_string() + &no_of_children.to_string();
		    	data_details = format!("{}\n{}\n{}\n{}{}",sibling_name_2, sibling_age_2, relationship, city_of_residence_2, no_of_children_2);
		    	data.push_str(&data_details);
		    	data.push_str("-");
		    }
		    if marital_status == 2 {
		    	relationship = "Marital Status: Single".to_string();
		    	let mut occupation = String::new();
		    	let mut job_description = String::new();
			    println!("Enter 1 if a worker and 2 if a student");
			    io::stdin().read_line(&mut occupation).expect("Failed to read input");
			    let occupation:i32 = occupation.trim().parse().expect("Not a valid integer");
		    	
		    	if occupation == 2 {
		    		job_description = "Occupation: Student".to_string();
		    		let mut university = String::new();
					println!("What university does he/she attend?");
				 	io::stdin().read_line(&mut university).expect("Failed to read input");
				 	let mut university_2 = String::new();
				 	university_2 = "University:".to_string() + &university;
				  	
				  	let mut course = String::new();
					println!("What course is he/she studying?");
				 	io::stdin().read_line(&mut course).expect("Failed to read input");
				 	let mut course_2 = String::new();
				 	course_2 = "Course:".to_string() + &course;
			  	
			  		data_details = format!("{}\n{}\n{}\n{}\n{}{}",sibling_name_2, sibling_age_2, relationship, job_description, university_2, course_2);
		    		data.push_str(&data_details);
		    		data.push_str("-");
		    	}

		    	if occupation == 1 {
		    		job_description = "Occupation: Worker".to_string();

		    		data_details = format!("{}\n{}\n{}\n{} ",sibling_name_2, sibling_age_2, relationship, job_description);
		    		data.push_str(&data_details);
		    		data.push_str("-");
		    	}
		    }
	  	}

	  	if sibling_age < 18 {
	  		let mut waec_status = String::new();
	  		let mut written_waec = String::new();
			println!("Has he/she written WAEC? \nEnter 1 for yes & 2 for no");
		    io::stdin().read_line(&mut waec_status).expect("Failed to read input");
		    let waec_status:i32 = waec_status.trim().parse().expect("Not a valid integer");
		    
		    if waec_status == 1 {
		    	written_waec = "WAEC Status: He/She has written WAEC.".to_string();
		    	let mut secondary_school_attended = String::new();
				println!("What secondary_school did he/she attend?");
			 	io::stdin().read_line(&mut secondary_school_attended).expect("Failed to read input");	
			 	let mut secondary_school_attended_2 = String::new();
			 	secondary_school_attended_2 = "Secondary School Attended:".to_string() + &secondary_school_attended;
		    	
		    	data_details = format!("{}\n{}\n{}\n{}",sibling_name_2, sibling_age_2, written_waec, secondary_school_attended_2);
		    	data.push_str(&data_details);
		    	data.push_str("-");
		   }

		    if waec_status == 2 {
		    	written_waec = "WAEC Status: He/She has NOT written WAEC.".to_string();
		    	let mut current_class_level = String::new();
		    	let mut current_class_level_2 = String::new();
				println!("What class is he/she in?");
			 	io::stdin().read_line(&mut current_class_level).expect("Failed to read input");
			 	current_class_level_2 = "Current Class Level:".to_string() + &current_class_level;
			 	
			 	data_details = format!("{}\n{}\n{}\n{}",sibling_name_2, sibling_age_2, written_waec, current_class_level);
		    	data.push_str(&data_details);
		    	data.push_str("-");
		    }
	  	}
	    }
    }

    // sibling_name_2, relationship, job_description, etc were created to make the printing of the data organized.
    let final_data = data.split("-");
	for i in final_data{
	 	println!("\n{}\n", i); 	
	}
}

