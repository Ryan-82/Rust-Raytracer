use crate::Color;
use crate::Vector;
use crate::Ray;
use crate::HitRecord;

use crate::random_f64_in_range;

//This redirects the scatter call to wherever its supposed to go.
#[derive(Clone, Copy)]
pub enum MaterialEnum {
	Lambertian(LambertianMaterial),
	Metal(MetalMaterial),
	Dielectric(DielectricMaterial),
}

impl MaterialEnum {
	pub fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		match self {
			MaterialEnum::Lambertian(material) => {
				//println!("noo");
				material.scatter(ray_in, rec, attenuation, scattered)
			}
			MaterialEnum::Metal(material) => {
				//println!("woo");
				material.scatter(ray_in, rec, attenuation, scattered)
			}
			MaterialEnum::Dielectric(material) => {
				material.scatter(ray_in, rec, attenuation, scattered)
			}
		}
	}

	pub fn new_metal(i: f64, j: f64, k: f64, fuzz: f64) -> MaterialEnum {
		MaterialEnum::Metal(MetalMaterial::new(Vector::new(i, j, k), fuzz))
	}
	pub fn new_lambertian(i: f64, j: f64, k: f64) -> MaterialEnum {
		MaterialEnum::Lambertian(LambertianMaterial::new(Vector::new(i, j, k)))
	}
	pub fn new_dielectric(index: f64) -> MaterialEnum {
		MaterialEnum::Dielectric(DielectricMaterial::new(index))
	}
}

impl Default for MaterialEnum {
	fn default() -> Self { MaterialEnum::Lambertian(LambertianMaterial::new(Color::unit_vector() * 0.5)) }
}

//This only exists to make sure that the function's parameters are adhered to.
trait Material {
	fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

//LAMBERTIAN MATERIAL

#[derive(Clone, Copy)]
pub struct LambertianMaterial {
	albedo: Color,
}

impl LambertianMaterial {
	pub fn new(albedo: Color) -> LambertianMaterial {
		LambertianMaterial {
			albedo,
		}
	}
}

impl Material for LambertianMaterial {
	fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let mut scatter_direction:Vector = rec.normal + Color::random_normal_vector();
		if scatter_direction.near_zero() {
			scatter_direction = rec.normal;
		}

		*scattered = Ray::new(rec.hit_point, scatter_direction);
		*attenuation = self.albedo;
		//println!("boop");
		return true;
	}
}

//METAL MATERIAL

#[derive(Clone, Copy)]
pub struct MetalMaterial {
	albedo: Color,
	fuzz: f64,
}

impl MetalMaterial {
	pub fn new(albedo: Color, fuzz: f64) -> MetalMaterial {
		MetalMaterial {
			albedo,
			fuzz,
		}
	}
}

impl Material for MetalMaterial {
	fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let reflected: Vector = Vector::reflect(&ray_in.dir, &rec.normal) + (self.fuzz * Vector::random_normal_vector());
		*scattered = Ray::new(rec.hit_point, reflected);
		*attenuation = self.albedo;
		// println!("boop");
		return true;
	}
}

#[derive(Clone, Copy)]
pub struct DielectricMaterial {
	refraction_index: f64,
}

impl DielectricMaterial {
	pub fn new(index: f64) -> DielectricMaterial {
		DielectricMaterial {
			refraction_index: index,
		}
	}

	fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
		let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
		r0 = r0 * r0;
		return r0 + (1.0 - r0) * f64::powi((1.0 - cosine), 5);
	}
}

impl Material for DielectricMaterial {
	fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		*attenuation = Color::new(1.0, 1.0, 1.0);
		let mut ri: f64 = self.refraction_index;
		if rec.front_face {
			ri = 1.0 / self.refraction_index;
		}

		let unit_direction = ray_in.dir.normalize();
		let cos_theta = Vector::dot(&(unit_direction * (-1.0)), &rec.normal).min(1.0);
		let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

		//let cannot_refract: bool = ri * sin_theta > 1.0;
		let mut direction: Vector = Vector::unit_vector();

		if ri * sin_theta > 1.0 || Self::reflectance(cos_theta, ri) > random_f64_in_range(0.0, 1.0) {
			direction = Vector::reflect(&unit_direction, &rec.normal);
		} else {
			direction = Vector::refract(&unit_direction, &rec.normal, ri);
		}


		*scattered = Ray::new(rec.hit_point, direction);
		return true;
	}
}