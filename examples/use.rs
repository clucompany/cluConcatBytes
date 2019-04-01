
#![feature(plugin)]
#![plugin(cluConcatBytes)]

fn main() {
	let c_str = concat_bytes!("cluWorld");
	//array: [99, 108, 117, 87, 111, 114, 108, 100], len: 8
	assert_eq!(c_str, b"cluWorld");
	
	
	let c_str2 = concat_bytes!("cluWorld");
	//array: [99, 108, 117, 87, 111, 114, 108, 100], len: 8
	assert_eq!(c_str2, b"cluWorld");
	
	
	let c_str3 = concat_bytes!("clu", b"World");
	//array: [99, 108, 117, 87, 111, 114, 108, 100], len: 8
	assert_eq!(c_str3, b"cluWorld");
	
	let c_str4 = concat_bytes!(
		b'c', b'l', b'u',
		b'W', b'o', b'r', b'l', b'd'
	);
	//array: [99, 108, 117, 87, 111, 114, 108, 100], len: 8
	assert_eq!(c_str4, b"cluWorld");
	
	let c_str5 = concat_bytes!(
		"clu",
		//It is possible to insert such values as: [u8], & 'static str, u8, i8, (0 without specifying the type).
		
		b'W', b'o', b'r', b'l', b"d"
		//The zero byte is automatically added, it is possible to write it, and it is possible not to write.
		//It is forbidden to insert zero byte in the middle or at the beginning of a line.
	);
	//array: [99, 108, 117, 87, 111, 114, 108, 100], len: 8
	assert_eq!(c_str5, b"cluWorld");
	
	my_function(c_str);
	my_function(c_str2);
	my_function(c_str3);
	my_function(c_str4);
	my_function(c_str5);
}

fn my_function(array:  &'static [u8]) {
	//'static --> it is possible not to write.
	
	println!("array: {:?}, len: {}", array, array.len());
}