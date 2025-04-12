use Color;
use Vector
use Ray;
use HitRecord;

struct Lambertian_material {
	albedo: Color,
}

impl Lambertian_material {
	fn new(albedo: Color) -> Lambertian_material {
		Lambertian_material {
			albedo,
		}
	}

	fn scatter(ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
		let scatter_direction:Vector = rec.normal + Color::random_normal_vector();
		if scatter_direction.near_zero() {
			scatter_direction = rec.normal;
		}
		
		scattered = ray(rec.origin, scatter_direction);
		attenuation = albedo;
		return true;
	}
}