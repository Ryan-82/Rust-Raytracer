//src/hittable
use crate::Vector;
use crate::Point;
use crate::Ray;
use crate::Interval;

#[derive(Debug, Default, Copy, Clone)]
pub struct HitRecord {
	pub hit_point: Point,
	pub normal: Vector,
	front_face: bool,
	t: f64,
}

impl HitRecord {
	//ensures that the normal is facing outwards and not inwards.
	fn set_face_normal (&mut self, r: &Ray, outward_normal: &Vector) {
		if Point::dot(&r.dir, &outward_normal) < 0.0 {
			self.front_face = true;
			self.normal = *outward_normal;
		}
		else {
			self.front_face = false;
			self.normal = *outward_normal * -1.0;
		}
	}
}

pub trait Hittable {
	fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
	center: Point,
	radius: f64,
}

impl Sphere {
	pub fn new (center: Point, radius: f64) -> Sphere {
		let mut checked_radius: f64 = 0.0;
		if radius > checked_radius {
			checked_radius = radius;
		}
		Sphere {
			center,
			radius: checked_radius,
		}
	}
}


//This is the specific hit function for the Sphere.
//It gets called by the HittableList, since each shape would have a diff func
impl Hittable for Sphere {
	//used the x^2 + y^2 = r^2 formula to find the hitpoint.
	fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
		let oc: Vector = self.center - r.origin;
		let a: f64 = r.dir.length_squared();
		let h: f64 = Point::dot(&r.dir, &oc);
		let c: f64 = oc.length_squared() - (self.radius * self.radius);
		let discriminant: f64 = h * h - a * c;

		if discriminant < 0.0 {
			return false;
		}
		//return ( h - f64::sqrt(discriminant)) / a; 
		let sqrtd: f64 = f64::sqrt(discriminant);

		let mut root: f64 = (h - sqrtd) / a;
		if !ray_t.surrounds(root) {
			root = (h + sqrtd) / a;
			if !ray_t.surrounds(root) {
				return false;
			}
		}

		rec.t = root;
		rec.hit_point = r.at(rec.t);
		//the normal is a unit vector.
		rec.set_face_normal(&r, &((rec.hit_point - self.center) / self.radius));

		return true;
	}
}

//#[derive(Debug, Copy, Clone)]
pub struct HittableList {
	objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
	pub fn new() -> HittableList {
		HittableList {
			objects: Vec::new(),
		}
	}
	pub fn add(&mut self, object: Box<dyn Hittable>) {
		self.objects.push(object);
	}

	// This goes through all the existing objects for every ray, and figures out where the closest point hit was.
	//and since t_max is closest_so_far, objects dont get considered unless theyre closer than the closest_so_far.

	//This can check multiple surfaces/spheres
	pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
		let mut temp_rec: HitRecord = HitRecord::default();
		let mut hit_anything: bool = false;
		let mut closest_so_far: f64 = ray_t.max;

		for object in &self.objects {
			if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
				hit_anything = true;
				closest_so_far = temp_rec.t;
				*rec = temp_rec;
			}
		}

		return hit_anything;
	}
}