use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    data: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { data: [x, y, z] }
    }

    pub fn from_array(data: [f64; 3]) -> Self {
        Self { data }
    }

    pub fn x(self) -> f64 {
        self.data[0]
    }

    pub fn y(self) -> f64 {
        self.data[1]
    }

    pub fn z(self) -> f64 {
        self.data[2]
    }

    pub fn len(self) -> f64 {
        self.len_sqr().sqrt()
    }

    pub fn len_sqr(self) -> f64 {
        self.data.iter().fold(0.0, |acc, el| acc + el * el)
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_array(self.data.map(|x| -x))
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.data[0] += rhs.x();
        self.data[1] += rhs.y();
        self.data[2] += rhs.z();
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.data[0] -= rhs.x();
        self.data[1] -= rhs.y();
        self.data[2] -= rhs.z();
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_array(self.data.map(|x| x * rhs))
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.data.each_mut().map(|x| *x *= rhs);
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::from_array(self.data.map(|x| x / rhs))
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.data.each_mut().map(|x| *x /= rhs);
    }
}
