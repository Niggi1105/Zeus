use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialOrd
        + PartialEq,
{
    x: F,
    y: F,
    z: F,
}

impl<F> Mul<F> for Vector3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialOrd
        + PartialEq,
{
    type Output = Self;
    fn mul(self, rhs: F) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl<F> MulAssign<F> for Vector3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialOrd
        + PartialEq,
{
    fn mul_assign(&mut self, rhs: F) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl<F> Div<F> for Vector3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialOrd
        + PartialEq,
{
    type Output = Self;
    fn div(self, rhs: F) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl<F> DivAssign<F> for Vector3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialOrd
        + PartialEq,
{
    fn div_assign(&mut self, rhs: F) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

//standard scalar product for vectors in R3
impl<F> Mul for Vector3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialOrd
        + PartialEq,
{
    type Output = F;
    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<F> Add for Vector3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialOrd
        + PartialEq,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl<F> AddAssign for Vector3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialOrd
        + PartialEq,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl<F> Sub for Vector3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialOrd
        + PartialEq,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl<F> SubAssign for Vector3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialOrd
        + PartialEq,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl<F> Vector3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialOrd
        + PartialEq,
{
    pub fn crossp(&self, rhs: &Self) -> Self {
        Vector3 {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }

    pub fn len2(&self) -> F {
        (*self) * (*self)
    }

    pub fn len(&self) -> f32
    where
        F: Into<f32>,
    {
        let l = self.len2().into();
        l.sqrt()
    }

    pub fn normalized(&self) -> Self
    where
        F: From<f32> + Into<f32>,
    {
        *self / self.len().into()
    }

    ///othogonal projection of self onto other
    pub fn proj(&self, other: &Self) -> Self {
        let t = (*self * *other) / (*other * *other);
        *other * t
    }
}
