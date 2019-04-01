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
