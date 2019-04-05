
#![feature(plugin)]
#![plugin(cluConcatBytes)]

fn main() {
	let c_str = concat_bytes!(@"cluWorld");
	//[u8; 8]
	//array: [99, 108, 117, 87, 111, 114, 108, 100], len: 8
	assert_eq!(&c_str, b"cluWorld");
	
	
	let c_str2 = concat_bytes!(@"cluWorld");
	//[u8; 8]
	//array: [99, 108, 117, 87, 111, 114, 108, 100], len: 8
	assert_eq!(&c_str2, b"cluWorld");
	
	
	let c_str3 = concat_bytes!(@"clu", b"World");
	//[u8; 8]
	//array: [99, 108, 117, 87, 111, 114, 108, 100], len: 8
	assert_eq!(&c_str3, b"cluWorld");
	
	my_function(c_str);
	my_function(c_str2);
	my_function(c_str3);
}

fn my_function(array:  [u8; 8]) {
	//'static --> it is possible not to write.
	
	println!("array: {:?}, len: {}", array, array.len());
}