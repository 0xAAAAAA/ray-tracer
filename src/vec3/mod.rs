use std::f32;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::marker::Copy;
use std::clone::Clone;

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn normalize(&self) -> Vec3 {
        let mg: f32 = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3::new(self.x / mg, self.y / mg, self.z / mg)
    }

    pub fn dot(self, a: Vec3) -> f32 {
        self.x * a.x + self.y * a.y + self.z * a.z
    }

    pub fn sdot(&self, a: &Vec3) -> f32 {
        self.x * a.x + self.y * a.y + self.z * a.z
    }
}

//////////////////////////////////////////////////

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self::new(self.x + other, self.y + other, self.z + other)
    }
}

//////////////////////////////////////////////////

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self::new(self.x - other, self.y - other, self.z - other)
    }
}

//////////////////////////////////////////////////

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Self::new(self.x / other, self.y / other, self.z / other)
    }
}

impl Div<i32> for Vec3 {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        Self::new(
            self.x / other as f32,
            self.y / other as f32,
            self.z / other as f32,
        )
    }
}

//////////////////////////////////////////////////

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul<i32> for Vec3 {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self::new(
            self.x * other as f32,
            self.y * other as f32,
            self.z * other as f32,
        )
    }
}

//////////////////////////////////////////////////

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Copy for Vec3 {}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        *self
    }
}
