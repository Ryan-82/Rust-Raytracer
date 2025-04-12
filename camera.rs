use crate::hittable::*;
use crate::Color;
use crate::Ray;
use crate::Point;
use crate::Vector;
use crate::Interval;

use std::fs::File;
use std::io::prelude::*;

use crate::color::write_color;

use crate::random_f64;

pub struct Camera {
	pub aspect_ratio: f64,
	pub image_width: i32,
	pub samples_per_pixel: i32,

	pixel_color_scale: f64,
	max_depth: i32,
	image_height: i32,
	center: Point,
	pixel00_loc: Point,
	pixel_delta_u: Vector,
	pixel_delta_v: Vector,
}

impl Camera {
	pub fn render(&self, world: &HittableList, file: &mut File) {
		//self.initialize();

		let _ = file.write_all(format!("P3\n{0} {1}\n255\n", self.image_width, self.image_height).as_bytes());

		for j in 0..self.image_height {
			println!("Scanlines Remaining: {}", (self.image_height - j));
			for i in 0..self.image_width {
				let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
				for sample in 0..self.samples_per_pixel {
					let r: Ray = self.get_ray(i, j);
					pixel_color += Self::ray_color(&r, self.max_depth, world);
				}
				pixel_color = pixel_color * self.pixel_color_scale;
				write_color(&pixel_color, file);
			}
		}
	}

	pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32) -> Camera {

		let pixel_color_scale = 1.0 / samples_per_pixel as f64;

		let mut image_height = (image_width as f64 / aspect_ratio) as i32;
		if image_height < 1 {
			image_height = 1;
		}

		let center = Point::null_vector();

		// Determine viewport dimensions
		let focal_length = 1.0;
		let viewport_height = 2.0;
		let viewport_width = viewport_height * aspect_ratio;

		// Calculate the vectors across the horizontal and down the vertical viewport edges
		let viewport_u = Vector::pos_x_vector() * viewport_width;
		let viewport_v = Vector::neg_y_vector() * viewport_height;

		// Calculate the horizontal and vertical delta vectors from pixel to pixel
		let pixel_delta_u = viewport_u / image_width;
		let pixel_delta_v = viewport_v / image_height;

		let viewport_upper_left: Point = center - Point::new(0.0, 0.0, focal_length) - (viewport_u/2) - (viewport_v/2);
		let pixel00_loc: Point = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


		Camera {
			aspect_ratio,
			image_width,
			samples_per_pixel,
			pixel_color_scale,
			image_height,
			max_depth: 10,
			center,
			pixel00_loc,
			pixel_delta_u,
			pixel_delta_v,
		}
	}

	fn ray_color(r: &Ray, depth: i32, world: &HittableList) -> Color {
		//let t = hit_sphere(&Point::new(0.0 , 0.0, -1.5), 0.5, r);
		if depth <= 0 {
			return Color::null_vector();
		}
		let mut rec = HitRecord::default();

		if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
			//return 0.5 * (rec.normal + Color::unit_vector());
			//let direction: Vector = Vector::random_on_hemisphere(&rec.normal);
			let direction: Vector = rec.normal + Vector::random_normal_vector();
			return 0.5 * Self::ray_color(&Ray::new(rec.hit_point, direction), depth - 1, world);
		}
		/*if t > 0.0 {
			let n: Vector = (r.at(t) - Vector::new(0.0,0.0,-1.5)).normalize();
			return 0.5 * (n + Color::unit_vector());
		}*/

		let unit_dir : Vector = Vector::normalize(&r.dir);
		let a: f64 = 0.5 * (unit_dir.y + 1.0);
		return ((1.0 - a) * Color::new(1.0,1.0,1.0)) + (a * Color::new(0.5, 0.7, 1.0));
	}

	fn get_ray(&self, i: i32, j: i32) -> Ray {
		/*
		let pixel_center: Vector = self.pixel00_loc + (i * self.pixel_delta_u) + (j * self.pixel_delta_v);
		let ray_direction: Vector = pixel_center - self.center;

		let r: Ray = Ray::new(self.center, ray_direction);

		let pixel_color: Color = Camera::ray_color(&r, world);
		write_color(&pixel_color, file);*/

		let offset: Point = Self::sample_square();
		let pixel_sample = self.pixel00_loc + ((i as f64 + offset.x) * self.pixel_delta_u) + ((j as f64 + offset.y) * self.pixel_delta_v);

		let ray_origin: Point = self.center;
		let ray_direction: Vector = pixel_sample - ray_origin;

		return Ray::new(ray_origin, ray_direction);
	}

	fn sample_square() -> Point {
		return Vector::new(random_f64() - 0.5, random_f64() - 0.5, 0.0) * 0.25;
	}
}