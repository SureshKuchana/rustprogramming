pub fn say_hello(){
	println!(" say_hello ");
}

pub fn print(){
	let nums = [1, 2, 3, 4, 5];
	for num in nums.iter() {
		println!("{}" , num);
	}
}