//#![allow(unused_imports)]
//#![allow(dead_code)]
//file sec/main.rs
mod vector;
use vector::Vector;

mod ray;
use ray::Ray;

mod color;
use color::*;

mod hittable;
use crate::hittable::*;

mod interval;
use interval::Interval;

mod camera;
use camera::*;

use std::fs::File;
use std::io::prelude::*;

use rand::Rng;
pub type Color = Vector;
pub type Point = Vector;


//UTILITY

static INFINITY: f64 = f64::INFINITY;
static PI: f64 = 3.1415926535897932385;


static EMPTY: Interval = Interval{
	min: f64::INFINITY,
	max: f64::INFINITY,
};
static UNIVERSE: Interval = Interval{
	min: f64::INFINITY,
	max: f64::INFINITY,
};


pub fn degrees_to_radians(degrees: f64) -> f64 {
	return (degrees * PI) / 180.0;
}

pub fn random_f64() -> f64 {
	return rand::random_range(0.0..1.0);
}
pub fn random_f64_in_range(min: f64, max: f64) -> f64 {
	return rand::random_range(min..max);
}

//MAIN
fn main() -> std::io::Result<()> {

	//Image
	let aspect_ratio: f64 = 16.0 / 9.0;

	let image_width: i32 = 400;

	let samples_per_pixel: i32 = 100;


	// Initialization
	let mut file = File::create("img.ppm")?;
	//Render
	//Camera
	let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel);

	

	
	//let _ = file.write_all(b"P3\n256 256\n255\n");
	//let _ = file.write_all(b"P3\n711 400\n255\n");
	//let _ = file.write_all(format!("P3\n{0} {1}\n255\n", image_width, image_height).as_bytes());
	//pixel_delta_u.print();
	//pixel_delta_v.print();


	let mut world: HittableList = HittableList::new();
	world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
	world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

	camera.render(&world, &mut file);

	//let test_vec = Vector::new(1.0, 1.0, 0.0);
	//test_vec.print();
	//println!("{}", test_vec.length_squared());
	Ok(())
}