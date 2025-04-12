//use Vector;
use crate::Point;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
	pub origin: Point,
	pub dir: Point,
}

impl Ray {
	pub fn at(&self, t: f64) -> Point {
		return self.origin + (t * self.dir);
	}
	pub fn new(origin: Point, dir: Point) -> Ray {
		Ray {
			origin,
			dir,
		}
	}
}