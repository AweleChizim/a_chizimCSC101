use std::io::Write;

fn main() {
	let lager = vec!["33 Export ", "Desperados", "Goldberg  ", "Guilder   ", "Heineken  ", "Star      "];
	let stout = vec!["Legend    ", "Turbo King", "Williams  ", "          ", "          ", "          "];
	let non_alcoholic = vec!["Maltina   ", "Amstel Malta", "Malta Gold", "Fayrouz   ", "          ", "          "];

	let mut nbl_file = std::fs::File::create("Nigerian_Brewery_Limited.txt").expect("create failed");
	nbl_file.write_all("LAGER      |   STOUT    | NON-ALCOHOLIC\n".as_bytes()).expect("write failed");
	nbl_file.write_all("_______________________________________\n".as_bytes()).expect("write failed");

	for i in 0..stout.len(){
		nbl_file.write_all(lager[i].as_bytes()).expect("write failed");
		nbl_file.write_all(" | ".as_bytes()).expect("write failed");
		nbl_file.write_all(stout[i].as_bytes()).expect("write failed");
    	nbl_file.write_all(" | ".as_bytes()).expect("write failed");
    	nbl_file.write_all(non_alcoholic[i].as_bytes()).expect("write failed");
    	nbl_file.write_all("\n".as_bytes()).expect("write failed");
	}

}
