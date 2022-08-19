use std::{fmt, ops};
use std::env::var;

#[derive(Debug)]
pub struct Vec3 {
    e: [f32; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn zero() -> Vec3 {
        return Vec3 { e: [0.0, 0.0, 0.0] };
    }

    pub fn from(x: f32, y: f32, z: f32) -> Vec3 {
        return Vec3 { e: [x, y, z] };
    }

    pub fn from(x: u32, y: u32, z: u32) -> Vec3 {
        return Vec3 { e: [x as f32, y as f32, z as f32] };
    }

    pub fn x(&self) -> f32 { return self.e[0]; }
    pub fn y(&self) -> f32 { return self.e[1]; }
    pub fn z(&self) -> f32 { return self.e[2]; }

    pub fn length_squared(&self) -> f32 {
        return (self.e[0] * self.e[0])
            + (self.e[1] * self.e[1])
            + (self.e[2] * self.e[2]);
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn unit(&self) -> Vec3 {
        let len = self.length();
        return self / len;
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        return Vec3 {e: self.e}
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Vec3::from(-self.e[0], -self.e[1], -self.e[2]);
    }
}

//region Add
impl <'a> ops::Add<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &'a Vec3) -> Self::Output {
        return Vec3::from(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]);
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        return Vec3::from(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]);
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        return Vec3::from(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]);
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(mut self, rhs: Vec3) -> Self::Output {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
        return self;
    }
}

impl ops::Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(mut self, rhs: f32) -> Self::Output {
        self.e[0] += rhs;
        self.e[1] += rhs;
        self.e[2] += rhs;
        return self;
    }
}

impl ops::Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, mut rhs: Vec3) -> Self::Output {
        rhs.e[0] += self;
        rhs.e[1] += self;
        rhs.e[2] += self;
        return rhs;
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}
//endregion

// region Mul
impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        return Vec3::from(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs);
    }
}

impl ops::Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        return Vec3::from(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs);
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        return Vec3::from(rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self);
    }
}

impl ops::Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        return Vec3::from(rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self);
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}
//endregion

//region Div
impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        return Vec3::from(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        return self;
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        return Vec3::from(rhs.e[0] / self, rhs.e[1] / self, rhs.e[2] / self)
    }
}

impl ops::Div<&Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output {
        return Vec3::from(rhs.e[0] / self, rhs.e[1] / self, rhs.e[2] / self)
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}
//endregion
