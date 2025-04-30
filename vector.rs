use std::ops::{Add, Sub, AddAssign, Index, IndexMut, Mul, MulAssign, Div, DivAssign};
use crate::random_f64_in_range;

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Vector {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}
impl Vector {
	pub fn new(x: f64, y: f64, z: f64) -> Vector {
		Vector {
			x,
			y,
			z,
		}
	}
	pub fn length_squared(&self) -> f64 {
		return (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
	}
	pub fn length(&self) -> f64 {
		f64::sqrt(self.length_squared())
	}
	/*pub fn dot(&self, rhs: &Vector) -> f64 {
		return (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
	}*/
	pub fn dot(lhs: &Vector, rhs: &Vector) -> f64 {
		return (lhs.x * rhs.x) + (lhs.y * rhs.y) + (lhs.z * rhs.z)
	}
	pub fn cross(&self, rhs: &Vector) -> Vector {
		return Vector {
			x: (self.y * rhs.z) - (self.z * rhs.y),
			y: (self.z * rhs.x) - (self.x * rhs.z),
			z: (self.x * rhs.y) - (self.y * rhs.x),
		}
	}
	pub fn normalize(&self) -> Vector {
		let len = self.length();
		if len == 0.0 {
			return Vector::null_vector();
		}
		return *self / len;
	}
	pub fn print(&self){
		println!("{}", format!("x:{0}, y:{1}, z:{2}", self.x, self.y, self.z));
	}

	pub fn null_vector() -> Vector {
		return Vector::new(0.0, 0.0, 0.0);
	}
	pub fn unit_vector() -> Vector {
		return Vector::new(1.0, 1.0, 1.0);
	}
	pub fn pos_x_vector() -> Vector {
		return Vector::new(1.0, 0.0, 0.0);
	}
	pub fn neg_x_vector() -> Vector {
		return Vector::new(-1.0, 0.0, 0.0);
	}
	pub fn pos_y_vector() -> Vector {
		return Vector::new(0.0, 1.0, 0.0);
	}
	pub fn neg_y_vector() -> Vector {
		return Vector::new(0.0, -1.0, 0.0);
	}
	pub fn pos_z_vector() -> Vector {
		return Vector::new(0.0, 0.0, 1.0);
	}
	pub fn neg_z_vector() -> Vector {
		return Vector::new(0.0, 0.0, -1.0);
	}

	pub fn random_vector() -> Vector {
		return Vector {
			x: random_f64_in_range(-1.0,1.0),
			y: random_f64_in_range(-1.0,1.0),
			z: random_f64_in_range(-1.0,1.0),
		}
	}
	pub fn random_normal_vector() -> Vector {
		return Self::random_vector().normalize();
	}
	pub fn random_on_hemisphere(normal: &Vector) -> Vector {
		let on_unit_sphere = Self::random_normal_vector();
		if Self::dot(&on_unit_sphere, normal) > 0.0 {
			return on_unit_sphere;
		}
		return -1.0 * on_unit_sphere;
	}
	pub fn reflect(v: &Vector, n: &Vector) -> Vector {
		return *v - 2.0 * Vector::dot(v, n) * *n;
	}

	pub fn refract(uv: &Vector, n: &Vector, etai_over_etat: f64) -> Vector {
		let mut cos_theta: f64 = 1.0;
		{
			let temp: f64 = Vector::dot(&(*uv * (-1.0)), n);
			if temp < cos_theta {
				cos_theta = temp;
			}
		}
		let r_out_perpendicular = etai_over_etat * (*uv + *n * cos_theta);
		let r_out_parallel = *n * -f64::sqrt((1.0 - (r_out_perpendicular.length_squared())).abs());
		return r_out_perpendicular + r_out_parallel;
	}

	pub fn near_zero(&self) -> bool {
		let s: f64 = 0.00000001;
		return (self.x < s) && (self.y < s) && (self.z < s);
	}
}
// a + b
impl Add<Vector> for Vector {
	//add returns  Sefl::Output, so were telling it
	//that the output is Vector, it could be anything else too
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
	}
}
// a - b
impl Sub<Vector> for Vector {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
	}
}
// a += b
impl AddAssign<Vector> for Vector {
	fn add_assign(&mut self, other: Self) {
		*self = Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}
// a[]
impl Index<u32> for Vector {
	type Output = f64;

	fn index(&self, index: u32) -> &Self::Output {
		match index % 3 {
			0 => &self.x,
			1 => &self.y,
			2 => &self.z,
			other => panic!("Index out of bounds: Vector has 3 elements, but index was {}", other),
		}
	}
}
// &a[]

impl IndexMut<u32> for Vector {
	//type Output = f64;
	//extends Output from Index so it's not needed

	fn index_mut(&mut self, index: u32) -> &mut Self::Output {
		match index % 3 {
			0 => &mut self.x,
			1 => &mut self.y,
			2 => &mut self.z,
			other => panic!("Index out of bounds: Vector has 3 elements, but index was {}", other),
		}
	}
}
// a * b
impl Mul<Vector> for Vector {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		Self {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
	}
	//add one for i32 if needed
	//pub fn mul(self, rhs: i32) -> Self {
}
impl Mul<f64> for Vector {
	type Output = Self;
	fn mul(self, rhs: f64) -> Self::Output {
		Self {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
	}
}
impl Mul<i32> for Vector {
	type Output = Self;
	fn mul(self, rhs: i32) -> Self::Output {
		self * rhs as f64
	}
}
impl Mul<Vector> for f64 {
	type Output = Vector;
	fn mul(self, rhs: Vector) -> Vector {
		Vector {x: self * rhs.x, y: self * rhs.y, z: self * rhs.z}
	}

}
impl Mul<Vector> for i32 {
	type Output = Vector;
	fn mul(self, rhs: Vector) -> Vector {
		return self as f64 * rhs;
	}
}
//a *= v
impl MulAssign<f64> for Vector {
	fn mul_assign(&mut self, rhs: f64) {
		self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
	}
}
impl MulAssign<i32> for Vector {
	fn mul_assign(&mut self, rhs: i32) {
		self.x *= rhs as f64;
		self.y *= rhs as f64;
		self.z *= rhs as f64;
	}
}
// a / b
impl Div<f64> for Vector {
	type Output = Self;

	fn div(self, rhs: f64) -> Self::Output {
		return self * (1.0/rhs);
	}
}
impl Div<i32> for Vector {
	type Output = Self;

	fn div(self, rhs:i32) -> Self::Output {
		return self * (1.0/rhs as f64);
	}
}
// a /= b
impl DivAssign<f64> for Vector {
	fn div_assign(&mut self, rhs: f64) {
		*self *= 1.0/rhs;
	}
}
impl DivAssign<i32> for Vector {
	fn div_assign(&mut self, rhs: i32) {
		*self *= 1.0/rhs as f64;
	}
}




