
#![feature(plugin)]
#![plugin(cluConcatBytes)]

//ATTENTION!!!
//
//For display of panic 'RLS or Rust Check' it is required to uncomment functions.

fn main() {
	let _c_str4 = concat_bytes!(
		b'c', b'l', b'u', 0u8,
		b'W', b'o', b'r', b'l', b'd' //, <--- Add
		//0 <---
	);
	//PANIC! incorrect data!
}