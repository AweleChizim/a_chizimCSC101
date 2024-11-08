use std::io;
use std::io::Read;
use rusqlite::{param, Connection};

fn main() {
    println!("Welcome To Globacom!");
    
    let mut user = String::new();
    println!("Please input who user is. \nFor ADMINISTRATOR, type 1; \nFor PROJECT MANAGER, type 2; \nFor EMPLOYEE, type 3; \nFor CUSTOMER, type 4; \nFor VENDOR, type 5;");
    io::stdin().read_line(&mut user).expect("Failed to read input");
    let user:i32 = user.trim().parse().expect("Not a valid integer");

    if user == 1 {
    	administrator();
    	}

    if user == 2 {
    	project_manager();
    	}

    if user == 3 {
    	employee();
    	}

    if user == 4 {
    	customer();
    	}

    if user == 5 {
    	vendor();
    	}


fn administrator() {
	let mut sims = std::fs::File::open("globacom_dbase.sql").unwrap();
	let mut administrator = String::new();
	sims.read_to_string(&mut administrator).unwrap();
   	print!("{}", administrator);
}

fn project_manager() {
	let mut sims = std::fs::File::open("project_tb.sql").unwrap();
	let mut project_manager = String::new();
	sims.read_to_string(&mut project_manager).unwrap();
   	print!("{}", project_manager);
}

fn employee() {
	let mut sims = std::fs::File::open("staff_tb.sql").unwrap();
	let mut employee = String::new();
	sims.read_to_string(&mut employee).unwrap();
   	print!("{}", employee);
}

fn customer() {
	let mut sims = std::fs::File::open("customer_tb.sql").unwrap();
	let mut customer = String::new();
	sims.read_to_string(&mut customer).unwrap();
   	print!("{}", customer);
}

fn vendor() {
	let mut sims = std::fs::File::open("data_plan_tb.sql").unwrap();
	let mut vendor = String::new();
	sims.read_to_string(&mut vendor).unwrap();
    print!("{}", vendor);
}

// let conn = Connection::open_in_memory().unwrap();

//     conn.execute(
//         "CREATE TABLE people (
//                   id              INTEGER PRIMARY KEY,
//                   name            TEXT NOT NULL,
//                   time_created    TEXT NOT NULL
//                   )",
//         params![],
//     ).unwrap();

//     conn.execute("INSERT INTO people (name, time_created) VALUES (?, ?)",
//                  params!["Alice", "2022-01-01 12:00:00"]).unwrap();

//     let selected_people = conn.query_map("SELECT id, name, time_created from people", params![], |row| {
//         Ok((row.get(0)?, row.get(1)?, row.get(2)?))
//     }).unwrap();

//     for person in selected_people {
//         let (id, name, time_created) = person.unwrap();
//         println!("id: {}, name: {}, time_created: {}", id, name, time_created);
//     }


// }
