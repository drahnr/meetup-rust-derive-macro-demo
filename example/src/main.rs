#[macro_use]
extern crate goose_derive;

pub trait Goose {
	fn fly();
}

#[derive(Goose,Debug)]
struct SittingDuck {
	x : i32,
	y : i32,
}

// Manual implementation
// 
// impl TestPoint {
// 	pub	fn generate() {
// 		TestPoint {
// 			x : 0,
// 			y : 0,
// 		}
// 	}
// }

fn main() {
	SittingDuck::fly();
}
