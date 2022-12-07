fn main() {
	let mut colors = ["red", "green", "yellow", "white"];
    println!("\nOriginal array = {:?}", colors);

    let sliced_colors = &mut colors[1..3];
    println!("Slice1 = {:?}", sliced_colors);

    sliced_colors[1] = "purple";
    println!("Changes slice = {:?}", sliced_colors);
}
