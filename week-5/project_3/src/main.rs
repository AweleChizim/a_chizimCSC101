// Rust program to order food and calculate the total charges.

use std::io;

fn main() 
{	
	let mut sum:i32 = 0;

	let s_n:[i32;5]= [1, 2, 3, 4, 5];
	let food =["Poundo Yam/Edinkaiko Soup", "Fried Rice & Chicken      ", "Amala & Ewedu Soup       ", "Eba & Egusi Soup            ", "White Rice & Stew          " ];
	let prices:[i32;5]= [3_200, 3_000, 2_500, 2_000, 2_500];

	let mut receipt = String::new();
	let mut receipt_details = String::new();


	//Input Order
    println!("Please select your order");

    loop {
    	// Input food type 
    	// p = Poundo Yam/Edinkaiko Soup; f = Fried Rice & Chicken; a = Amala & Ewedu Soup; e = Eba & Egusi Soup; w = White Rice & Stew
    	let mut type_of_food = String::new();
    	println!("----------------------------------------------");
    	println!("\tFOOD \t\t\t\tPRICE");
    	println!("----------------------------------------------");
    	println!("----------------------------------------------");

    	for i in 0..food.len(){
    		println!("{} for {} \t {}", s_n[i], food[i], prices[i]);
    		println!("----------------------------------------------");
    	} 

    	io::stdin().read_line(&mut type_of_food).expect("Failed to read input");
    	let type_of_food:i32 =type_of_food.trim().parse().expect("Not a valid integer");
   
  		if type_of_food > 5 || type_of_food < 1
    	{
        	println!("Invalid input");
   		}
   		
    	else if type_of_food == 1
    	{
	    	// Input qty of 1/P to be ordered
	    	let mut quantity_p = String::new();
	   		println!("\nEnter the quantity of food you want to order:");
	   		io::stdin().read_line(&mut quantity_p).expect("Failed to read input");
	   		let quantity_p:i32 =quantity_p.trim().parse().expect("Not a valid integer");
	   		let p_o_p:i32  = prices[0] * quantity_p;
	   		sum = sum + p_o_p;
	    	println!("{} portions of Poundo Yam/Edinkaiko Soup = N{}", quantity_p, p_o_p); 
	    	receipt_details = format!("{}\t{}\t N{}", food[0], quantity_p, p_o_p); 
	    	receipt.push_str(&receipt_details);
	    	receipt.push_str("-");

   		}
		
    	else if type_of_food == 2
    	{
	    	// Input qty of 2/F to be ordered
	    	let mut quantity_f = String::new();
	   		println!("\nEnter the quantity of food you want to order:");
	   		io::stdin().read_line(&mut quantity_f).expect("Failed to read input");
	   		let quantity_f:i32 =quantity_f.trim().parse().expect("Not a valid integer");
	   		let p_o_f:i32  = prices[1] * quantity_f;
	   		sum = sum + p_o_f;
	    	println!("{} portions of Fried Rice & Chicken = N{}", quantity_f, p_o_f);   
	    	receipt_details = format!("{}\t{}\t N{}", food[1], quantity_f, p_o_f); 
	    	receipt.push_str(&receipt_details);
	    	receipt.push_str("-");
   		}

    	else if type_of_food == 3
    	{
	    	// Input qty of 3/A to be ordered
	    	let mut quantity_a = String::new();
	   		println!("\nEnter the quantity of food you want to order:");
	   		io::stdin().read_line(&mut quantity_a).expect("Failed to read input");
	   		let quantity_a:i32 =quantity_a.trim().parse().expect("Not a valid integer");
	   		let p_o_a:i32  = prices[2] * quantity_a;
	   		sum = sum + p_o_a;
	    	println!("{} portions of Amala & Ewedu Soup = N{}", quantity_a, p_o_a); 
	    	receipt_details = format!("{}\t{}\t N{}", food[2], quantity_a, p_o_a); 
	    	receipt.push_str(&receipt_details);
	    	receipt.push_str("-");
   		}

    	else if type_of_food == 4
    	{
	    	// Input qty of 4/E to be ordered
	    	let mut quantity_e = String::new();
	   		println!("\nEnter the quantity of food you want to order:");
	   		io::stdin().read_line(&mut quantity_e).expect("Failed to read input");
	   		let quantity_e:i32 =quantity_e.trim().parse().expect("Not a valid integer");
	   		let p_o_e:i32  = prices[3] * quantity_e;
	   		sum = sum + p_o_e;
	    	println!("{} portions of Eba & Egusi Soup = N{}", quantity_e, p_o_e); 
	    	receipt_details = format!("{}\t{}\t N{}", food[3], quantity_e, p_o_e); 
	    	receipt.push_str(&receipt_details);
	    	receipt.push_str("-");
   		}

    	else if type_of_food == 5
    	{
	    	// Input qty of 5/W to be ordered
	    	let mut quantity_w = String::new();
	   		println!("\nEnter the quantity of food you want to order:");
	   		io::stdin().read_line(&mut quantity_w).expect("Failed to read input");
	   		let quantity_w:i32 =quantity_w.trim().parse().expect("Not a valid integer");
	   		let p_o_w:i32  = prices[4] * quantity_w;
	   		sum = sum + p_o_w;
	    	println!("{} portions of White Rice & Stew = N{}", quantity_w, p_o_w); 
	    	receipt_details = format!("{}\t{}\t N{}", food[4], quantity_w, p_o_w); 
	    	receipt.push_str(&receipt_details);
	    	receipt.push_str("-");
   		}


   		// Input yes if you want to order anything else and no if you don't
		let mut order_again = String::new();
		println!("\nWould you like to order anything else? \nEnter 1 for Yes and any number for No");
		io::stdin().read_line(&mut order_again).expect("Failed to read input");
   		let order_again:i32 =order_again.trim().parse().expect("Not a valid integer");

	    if order_again > 1 || order_again == 0 {
	    	println!("Calculating total charge...");	    	
	    	break;}
	}
	// Receipt
	println!("RECEIPT");
	println!("\n\n");
	println!("--------------------------------------------------");
    println!("\tFOOD \t\t   QUANTITY\tPRICE");
    println!("--------------------------------------------------");
    let final_receipt = receipt.split("-");
	 for i in final_receipt{
	 	println!("--------------------------------------------------");
	 	println!("{}", i); 	
	 }
	println!("\t\t\t\tTOTAL \t N{}", sum);
	println!("--------------------------------------------------");
	if sum <= 10_000 {
		println!("DISCOUNT 0% \nTOTAL:-  N{}", sum);
    }
	if sum > 10_000 {
		let discount:i32 = sum * 5 / 100;
		let total_after_discount:i32 = sum - discount;
		println!("DISCOUNT 5% \nTOTAL:-  N{}", total_after_discount);
	}

	
	
}