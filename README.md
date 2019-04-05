# cluConcatBytes
[![Build Status](https://travis-ci.org/clucompany/cluConcatBytes.svg?branch=master)](https://travis-ci.org/clucompany/cluConcatBytes)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluConcatBytes)](https://crates.io/crates/cluConcatBytes)
[![Documentation](https://docs.rs/cluConcatBytes/badge.svg)](https://docs.rs/cluConcatBytes)

# Use

1. Easy use.

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