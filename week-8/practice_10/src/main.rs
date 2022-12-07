fn main() {
	let num = [1, 2, 3, 4, 5];
    println!("Original array = {:?}", num);

    let slice1 = &num[1..3];
    println!("slice1 = {:?}", slice1);
    //slice1 = [2,3]

    let slice2 = &num[..3];
    println!("slice2 = {:?}", slice2);
    //slice2S = [1,2,3]

    let slice3 = &num[2..];
    println!("slice3 = {:?}", slice3);
    //slice3 = [3,4,5] 

    let slice4 = &num[..];
    println!("slice4 = {:?}", slice4);
    //slice4 = [1,2,3,4,5] 
}
