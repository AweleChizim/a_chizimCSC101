use std::io::Write;

fn main() {
	let strategy:[&str;7] = ["Strategy consulting", "Corporate and growth strategy", "Transaction strategy and execution", 
	 "Restructuring and turnaround strategy", "Industry strategy", "Digital business building", "Commercial strategy"];
	let consulting:[&str;7] = ["Analytics consulting services", "Customer experience", "Cybersecurity,strategy,risk,compliance and resilience",
	 "Digital information", "Risk consulting services", "Supply chain and operations", "Technology transformation"];
	let p_a_w:[&str;7] = ["Change management and experience", "HR transformation", "Integrated workforce mobility", "Learning and development consulting",
     "Recognition and reward advisory", "Workforce analytics", "People and workforce"];
	let t_a_cf:[&str;7] = ["Corporate finance", "Divestments and carve-outs", "Sustainability and ESG Services", "M&A advisory",
	 "M&A integration", "M&A technology and tools", "M&A advanced analytics"];
	let tax:[&str;7] = ["Tax planning", "Tax function operations", "Tax policy and controversy", "Global trade",
	 "Tax accounting", "Tax compliance", "Transaction tax"];
	let assurance:[&str;7] = ["Audit services", "Climate change and sustainability services", "Financial accounting advisory services", 
	"Forensic and integrity services","Private client audit experience", "Accounting Link", "Assurance"];
	let s_name:[&str;6] = ["Aigbona_Juliet", "Ehis_Ero", "Adamu_Sagamu", "Akpevwe_Iloka", "Maria_Akinsola", "Gbenga_Daniels"];
	let s_dept:[&str;6] = ["Consulting", "Strategy", "Tax", "Assurance", "Transactions and corporate finance", "People and workforce"];
	let s_qualification:[&str;6] = ["B.Sc.","M.Sc.","B.Sc.","HND","M.Sc.","HND"];
	let s_code:[i32;6] = [7,9,8,7,9,8];
	
	code_7(strategy, consulting, p_a_w, t_a_cf, tax, assurance, s_name, s_dept, s_qualification, s_code);
	code_8(strategy, consulting, p_a_w, t_a_cf, tax, assurance, s_name, s_dept, s_qualification, s_code);
	code_9(strategy, consulting, p_a_w, t_a_cf, tax, assurance, s_name, s_dept, s_qualification, s_code);

    println!("Files Created!");
}

fn code_7(strategy:[&str;7], consulting:[&str;7], p_a_w:[&str;7], t_a_cf:[&str;7], tax:[&str;7], assurance:[&str;7], 
	s_name:[&str;6], s_dept:[&str;6], s_qualification:[&str;6], s_code:[i32;6]) {
	for i in 0..s_name.len()
	{
		if s_code[i] == 7 {

			if s_dept[i] == "Consulting" {
				let mut sims_file_1 = std::fs::File::create(s_name[i]).expect("create failed");
				sims_file_1.write_all("Qualification:- ".as_bytes()).expect("write failed");
				sims_file_1.write_all(s_qualification[i].as_bytes()).expect("write failed");
				sims_file_1.write_all("\n".as_bytes()).expect("write failed");
				sims_file_1.write_all("Department:- ".as_bytes()).expect("write failed");
				sims_file_1.write_all(s_dept[i].as_bytes()).expect("write failed");
				sims_file_1.write_all("\n".as_bytes()).expect("write failed");
				sims_file_1.write_all("Department Services:- ".as_bytes()).expect("write failed");
				for i in 0..6
				{
					sims_file_1.write_all(consulting[i].as_bytes()).expect("write failed");
					sims_file_1.write_all("; ".as_bytes()).expect("write failed");
				}
				sims_file_1.write_all(consulting[6].as_bytes()).expect("write failed");	
				sims_file_1.write_all(".".as_bytes()).expect("write failed");
			}

			else if s_dept[i] == "Assurance" {
				let mut sims_file_2 = std::fs::File::create(s_name[i]).expect("create failed");
				sims_file_2.write_all("Qualification:- ".as_bytes()).expect("write failed");
				sims_file_2.write_all(s_qualification[i].as_bytes()).expect("write failed");
				sims_file_2.write_all("\n".as_bytes()).expect("write failed");
				sims_file_2.write_all("Department:- ".as_bytes()).expect("write failed");
				sims_file_2.write_all(s_dept[i].as_bytes()).expect("write failed");
				sims_file_2.write_all("\n".as_bytes()).expect("write failed");
				sims_file_2.write_all("Department Services:- ".as_bytes()).expect("write failed");
				for i in 0..6
				{
					sims_file_2.write_all(assurance[i].as_bytes()).expect("write failed");
					sims_file_2.write_all("; ".as_bytes()).expect("write failed");
				}
				sims_file_2.write_all(assurance[6].as_bytes()).expect("write failed");	
				sims_file_2.write_all(".".as_bytes()).expect("write failed");
			}
		}
	}
}

fn code_8(strategy:[&str;7], consulting:[&str;7], p_a_w:[&str;7], t_a_cf:[&str;7], tax:[&str;7], assurance:[&str;7],
	s_name:[&str;6], s_dept:[&str;6], s_qualification:[&str;6], s_code:[i32;6]) {
	for i in 0..s_name.len()
	{
		if s_code[i] == 8 {
			if s_dept[i] == "Tax" {
				let mut sims_file_3 = std::fs::File::create(s_name[i]).expect("create failed");
				sims_file_3.write_all("Qualification:- ".as_bytes()).expect("write failed");
				sims_file_3.write_all(s_qualification[i].as_bytes()).expect("write failed");
				sims_file_3.write_all("\n".as_bytes()).expect("write failed");
				sims_file_3.write_all("Department:- ".as_bytes()).expect("write failed");
				sims_file_3.write_all(s_dept[i].as_bytes()).expect("write failed");
				sims_file_3.write_all("\n".as_bytes()).expect("write failed");
				sims_file_3.write_all("Department Services:- ".as_bytes()).expect("write failed");
				for i in 0..6
				{
					sims_file_3.write_all(tax[i].as_bytes()).expect("write failed");
					sims_file_3.write_all(", ".as_bytes()).expect("write failed");
				}
				sims_file_3.write_all(tax[6].as_bytes()).expect("write failed");	
				sims_file_3.write_all(".".as_bytes()).expect("write failed");
			}
			else if s_dept[i] == "People and workforce" {
				let mut sims_file_4 = std::fs::File::create(s_name[i]).expect("create failed");
				sims_file_4.write_all("Qualification:- ".as_bytes()).expect("write failed");
				sims_file_4.write_all(s_qualification[i].as_bytes()).expect("write failed");
				sims_file_4.write_all("\n".as_bytes()).expect("write failed");
				sims_file_4.write_all("Department:- ".as_bytes()).expect("write failed");
				sims_file_4.write_all(s_dept[i].as_bytes()).expect("write failed");
				sims_file_4.write_all("\n".as_bytes()).expect("write failed");
				sims_file_4.write_all("Department Services:- ".as_bytes()).expect("write failed");
				for i in 0..6
				{
					sims_file_4.write_all(p_a_w[i].as_bytes()).expect("write failed");
					sims_file_4.write_all(", ".as_bytes()).expect("write failed");
				}
				sims_file_4.write_all(p_a_w[6].as_bytes()).expect("write failed");	
				sims_file_4.write_all(".".as_bytes()).expect("write failed");
			}
		}
	}
}

fn code_9(strategy:[&str;7], consulting:[&str;7], p_a_w:[&str;7], t_a_cf:[&str;7], tax:[&str;7], assurance:[&str;7], 
	s_name:[&str;6], s_dept:[&str;6], s_qualification:[&str;6], s_code:[i32;6]) {
	for i in 0..s_name.len()
	{
		if s_code[i] == 9 {
			if s_dept[i] == "Strategy" {
				let mut sims_file_5 = std::fs::File::create(s_name[i]).expect("create failed");
				sims_file_5.write_all("Qualification:- ".as_bytes()).expect("write failed");
				sims_file_5.write_all(s_qualification[i].as_bytes()).expect("write failed");
				sims_file_5.write_all("\n".as_bytes()).expect("write failed");
				sims_file_5.write_all("Department:- ".as_bytes()).expect("write failed");
				sims_file_5.write_all(s_dept[i].as_bytes()).expect("write failed");
				sims_file_5.write_all("\n".as_bytes()).expect("write failed");
				sims_file_5.write_all("Department Services:- ".as_bytes()).expect("write failed");
				for i in 0..6
				{
					sims_file_5.write_all(strategy[i].as_bytes()).expect("write failed");
					sims_file_5.write_all(", ".as_bytes()).expect("write failed");
				}
				sims_file_5.write_all(strategy[6].as_bytes()).expect("write failed");	
				sims_file_5.write_all(".".as_bytes()).expect("write failed");
			}
			else if s_dept[i] == "Transactions and corporate finance" {
				let mut sims_file_6 = std::fs::File::create(s_name[i]).expect("create failed");
				sims_file_6.write_all("Qualification:- ".as_bytes()).expect("write failed");
				sims_file_6.write_all(s_qualification[i].as_bytes()).expect("write failed");
				sims_file_6.write_all("\n".as_bytes()).expect("write failed");
				sims_file_6.write_all("Department:- ".as_bytes()).expect("write failed");
				sims_file_6.write_all(s_dept[i].as_bytes()).expect("write failed");
				sims_file_6.write_all("\n".as_bytes()).expect("write failed");
				sims_file_6.write_all("Department Services:- ".as_bytes()).expect("write failed");
				for i in 0..6
				{
					sims_file_6.write_all(t_a_cf[i].as_bytes()).expect("write failed");
					sims_file_6.write_all(", ".as_bytes()).expect("write failed");
				}
				sims_file_6.write_all(t_a_cf[6].as_bytes()).expect("write failed");	
				sims_file_6.write_all(".".as_bytes()).expect("write failed");
			}
		}
	}
}

