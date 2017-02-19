extern crate bincode;
extern crate rustc_serialize;

use std::str::String;

macro_rules! burp {
	( $( $x:expr ),+ ) => {{
			let mut temp = Vec::with_capacity(1);
			$( temp.push($x); )+
			temp
	}}
}


let x = vec!["blah"];

macro_rules! struct_with_delta {
	($s:ident, $( $attr_name:ident :$attr_type:ty ),* ) => (
		// Struct has an internal mirror of all values
		#[derive(Debug)]
		struct Delta {
			$( $attr_name : $attr_type ),*
		}
		#[derive(Debug, Encodable, Decodable)]
		pub struct $s {
			delta: Delta,
			$( $attr_name : $attr_type ),*
		}
		impl $s {
			fn new( $( $attr_name : $attr_type ),* ) -> Self {
				$s{ 
					delta: Delta {
						$( $attr_name: $attr_name ),*,
					},
					$( $attr_name: $attr_name ),*,
				}
			}

			$(
				#[inline]
				fn $attr_name(&self) -> &$attr_type {
					&self.$attr_name
				}
			)*

			fn is_dirty(&self) -> bool {
				$(
					if self.$attr_name != self.delta.$attr_name {
						return true;
					}
				)*
				false
			}

		}
	)
}

struct Something {
	x: f32,
	y: f32
}

impl Something {
	fn new() -> Self {
		Something { x: 15.0, y:15.0 } 
	}
}

#[macro_export]
macro_rules! struct_with_ctor {
	($s:ident, $( $attr_name:ident :$attr_type:ty ),* ) => (
		#[derive(Debug)]
		struct $s {
			$($attr_name : $attr_type),*
		}

		impl $s {
			pub fn new( $( $attr_name : $attr_type ),* ) -> Self {
				$s{ $( $attr_name: $attr_name ),* }
			}
			$(
				#[inline]
				pub fn $attr_name(&self) -> $attr_type {
					self.$attr_name
				}
			)*
		}

	)
}

#[macro_export]
macro_rules! struct_with_field_list {
	($s:ident, $( $attr_name:ident :$attr_type:ty ),* ) => (
		struct_with_delta!( $s, $($attr_name : $attr_type),*);
		impl $s {
			fn fields(&self) -> Vec<&'static str> {
				let mut fields = Vec::new();
				$( fields.push( stringify!($attr_name) ); )*
				fields	
			}
		}
	)
}

macro_rules! match_some {
	($e:expr, $p: pat) => {{
		match $e {
			$p => println!("match"),
			_ => println!("Nothing")
		}
	}}
}

macro_rules! scope {
	($b: expr) => { $b }
}

struct_with_field_list!(
	Dollop,
	x: u32,
	y: f32,
	name: &'static str
);

impl Dollop {
	fn set_name(&mut self, value: &'static str) {
		self.name = value;
	}
}

fn main() {
    println!("Hello, world!");
		let temp = "A";
		let merp = burp![temp, "b"];
		let mut g = Dollop::new( 15, 3.0, "derp" );
		let gg = Dollop::new( 42, 39.2, "derp" );
		println!("{:?}", g);
		g.set_name("merp");
		println!("{:?}, {}", g, g.name());
		println!("is_dirty: {}", g.is_dirty());
		println!("is_dirty: {}", gg.is_dirty());

		println!("Dollop fields: {:?}", g.fields()); 

		scope!{{
			let x = 1;
			println!("inside scope {}", x);
		}};
}
