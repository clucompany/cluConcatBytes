//Copyright 2019 #UlinProject Денис Котляров

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.


//#Ulin Project 1819
//

/*!

Merges literals into a static array.

# Use

1. Easy use

```
#![feature(plugin)]
#![plugin(cluConcatBytes)]

fn main() {
	let c_str = concat_bytes!("cluWorld");
	//&'static [u8]
	//array: [99, 108, 117, 87, 111, 114, 108, 100], len: 8
	assert_eq!(c_str, b"cluWorld");
	
	
	let c_str2 = concat_bytes!("clu", b"World");
	//&'static [u8]
	//array: [99, 108, 117, 87, 111, 114, 108, 100], len: 8
	assert_eq!(c_str2, b"cluWorld");
	
	let c_str3 = concat_bytes!(
		b'c', b'l', b'u',
		b'W', b'o', b'r', b'l', b'd'
	);
	//&'static [u8]
	//array: [99, 108, 117, 87, 111, 114, 108, 100], len: 8
	assert_eq!(c_str3, b"cluWorld");
	
	let c_str4 = concat_bytes!(
		"clu", b'W', b'o', b'r', b'l', b"d"
	);
	//&'static [u8]
	//array: [99, 108, 117, 87, 111, 114, 108, 100], len: 8
	assert_eq!(c_str4, b"cluWorld");
	
	my_function(c_str);
	my_function(c_str2);
	my_function(c_str3);
	my_function(c_str4);
}

fn my_function(array:  &'static [u8]) {
	//'static --> it is possible not to write.
	
	println!("array: {:?}, len: {}", array, array.len());
}
```

2. Raw use

```
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
```

# License

Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

Licensed under the Apache License, Version 2.0
*/

#![feature(plugin_registrar)]
#![feature(rustc_private)]


extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;


mod nightly;
pub use self::nightly::*;

//The description only for documentation not to use. 
//Connect a plug-in! (The plug-in is described in nightly.rs, connect a plug-in as it is specified in documentation)
#[macro_export]
macro_rules! concat_bytes {
	($($s:expr),*) => {unimplemented!()};
	($s:expr) => {unimplemented!()};
	() => {unimplemented!()};
}


