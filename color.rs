use crate::Vector;
use crate::Ray;
use std::fs::File;
use std::io::prelude::*;
//COLOR

use crate::Color;
use crate::Point;
use crate::hittable::*;
use crate::Interval;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
	if linear_component > 0.0 {
		return f64::sqrt(linear_component);
	}

	return 0.0;
}

pub fn write_color(pixel_color: &Vector, file: &mut File) {
	let r: f64 = linear_to_gamma(pixel_color.x);
	let g: f64 = linear_to_gamma(pixel_color.y);
	let b: f64 = linear_to_gamma(pixel_color.z);


	//we use 255.999 because when r is 255. multiplying 256 makes it 256
	//as i32 removes the digits. and 255.999 ensures we gets some leeway
	//we can use 255. but its safer to use 255.999 because floating
	//point calculations sometimes have errors.
	//it handles an edge case for mapping to integers through truncation.
	let intensity: Interval = Interval::new(0.0, 0.999);
	let ir: i32 = (255.999 * intensity.clamp(r)) as i32;
	let ig: i32 = (255.999 * intensity.clamp(g)) as i32;
	let ib: i32 = (255.999 * intensity.clamp(b)) as i32;

	let temp_string: String = format!("{ir} {ig} {ib}\n");

	//println!("{ir}  {ig}  {ib} \n");
	let _ = file.write_all(temp_string.as_bytes());
}

fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> f64 {
	let oc: Vector = *center - r.origin;
	let a = r.dir.length_squared();
	let h = Point::dot(&r.dir, &oc);
	let c = oc.length_squared() - (radius * radius);
	let discriminant = h * h - a * c;

	if discriminant >= 0.0 {
		//r.dir.print();
		return ( h - f64::sqrt(discriminant)) / a; 
	}
	return -1.0;
}

/*
//color goes [0,1] since it gets multiplied by 255.999 later.
pub fn ray_color(r: &Ray, world: &HittableList) -> Color {
	//let t = hit_sphere(&Point::new(0.0 , 0.0, -1.5), 0.5, r);
	let mut rec = HitRecord::default();

	if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
		return 0.5 * (rec.normal + Color::unit_vector());
	}
	/*if t > 0.0 {
		let n: Vector = (r.at(t) - Vector::new(0.0,0.0,-1.5)).normalize();
		return 0.5 * (n + Color::unit_vector());
	}*/

	let unit_dir : Vector = Vector::normalize(&r.dir);
	let a: f64 = 0.5 * (unit_dir.y + 1.0);
	return ((1.0 - a) * Color::new(1.0,1.0,1.0)) + (a * Color::new(0.5, 0.7, 1.0));
}
*/