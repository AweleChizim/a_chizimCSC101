// CA_2

use std::io;

fn main()
{
	// main2aa(&faculty_name_name, &incentive_2);
	// let run_code = faculty_name_name + incentive_2;


	let faculty_members:i32 = 2;

	for i in 0..faculty_members{

		let mut faculty_name = String::new();
	    println!("Please enter your name");
	    io::stdin().read_line(&mut faculty_name).expect("Failed to read input");
	    
	    let mut no_of_papers = String::new();
	    println!("Please enter the number of papers you have published");
	    io::stdin().read_line(&mut no_of_papers).expect("Failed to read input");
	    let no_of_papers:i32 = no_of_papers.trim().parse().expect("Not a valid integer");

	    facpub(&faculty_name, no_of_papers, i);
	//     let final_data = data.split("-");
	//     for i in final_data{
	//  	println!("\n{}\n", i); 	
	// }	
	    
	}

	
}

/*fn main2aa(faculty_name_name: &str, incentive_2: &str){
	// studentcouncil_votex();


}
*/
fn studentcouncil_votex() {
	// Checks candidate requirements
	let eligible_candidates:i32 = 15;
	let mut data = String::new();
	let mut data_details = String::new();
	
		for i in 0..eligible_candidates {
    println!("Good day");
    let mut class_rep = String::new();
    println!("Are you a current Class Rep?\nEnter 1 for yes and 2 for no");
    io::stdin().read_line(&mut class_rep).expect("Failed to read input");
    let class_rep:i32 = class_rep.trim().parse().expect("Not a valid integer");

    let mut level = String::new();
    println!("Are you in 100level?\nEnter 1 for yes and 2 for no");
    io::stdin().read_line(&mut level).expect("Failed to read input");
    let level:i32 = level.trim().parse().expect("Not a valid integer");

	let mut cgpa = String::new();
    println!("Is your CGPA above 4.0?\nEnter 1 for yes and 2 for no");
    io::stdin().read_line(&mut cgpa).expect("Failed to read input");
    let cgpa:i32 = cgpa.trim().parse().expect("Not a valid integer");

    if (class_rep == 1) && (level == 2) && (cgpa == 1){
    		println!("You can vote!");
    		
		let mut name = String::new();
		println!("Type in your name:-");
	 	io::stdin().read_line(&mut name).expect("Failed to read input");
	  	
	  	let mut email = String::new();
		println!("Type in your E-mail:-");
	 	io::stdin().read_line(&mut email).expect("Failed to read input");
	  	
	  	let mut department = String::new();
		println!("Type in your Department:-");
	 	io::stdin().read_line(&mut department).expect("Failed to read input");
	  	
	  	let mut state_of_origin = String::new();
		println!("Type in your State of Origin:-");
	 	io::stdin().read_line(&mut state_of_origin).expect("Failed to read input");
	  	
	  	let mut name_2 = String::new();
	  	let mut email_2 = String::new();
	  	let mut department_2 = String::new();
	  	let mut state_of_origin_2 = String::new();

	  	name_2 = "Candidate Name:".to_string() + &name;
		email_2 = "Email:".to_string() + &email;
		department_2 = "Department:".to_string() + &department;
		state_of_origin_2 = "State of Origin:".to_string() + &state_of_origin;

		data_details = format!("{}\n{}\n{}\n{}",name_2, email_2, department_2, state_of_origin_2);
		data.push_str(&data_details);
    	data.push_str("-");
	}

	else {
   	println!("Sorry, you are not eligible to vote");
    }

}
	let final_data = data.split("-");
	for i in final_data{
	 	println!("\n{}\n", i); 	
	}	

    
    
}




fn facpub (faculty_name: &str, no_of_papers: i32, i: i32){
	// checks faculty requirements and prints incentive
	let mut incentive:i32 = 0;
	let mut faculty_name_name = "Name:".to_string() + &faculty_name;
	let mut data = String::new();
	let mut data_details = String::new();
	
			if no_of_papers < 3{
				incentive = 100_000;
				let ba = incentive.to_string();
				let incentive_2 = "Incentive: N".to_string() + &ba;
			}
			if no_of_papers >= 3 && no_of_papers <= 5{
			 	incentive = 500_000;
			 	let ba = incentive.to_string();
				let incentive_2 = "Incentive: N".to_string() + &ba;
			} 
			if no_of_papers > 5 && no_of_papers < 10{
				incentive = 800_000;
				let ba = incentive.to_string();
				let incentive_2 = "Incentive: N".to_string() + &ba;
			}
			if no_of_papers >= 10{
				incentive = 1_000_000;
				let ba = incentive.to_string();
				let incentive_2 = "Incentive: N".to_string() + &ba;
			}
			// data_details = format!("{}.{}\n{}", i, faculty_name_name, incentive_2);
		 //    data.push_str(&data_details);
   // 		    data.push_str("-");

   // 		    return data();



}
