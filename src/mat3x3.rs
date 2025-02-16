use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Matrix3x3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialEq
        + Default,
{
    m: [[F; 3]; 3],
}

impl<F> Add for Matrix3x3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialEq
        + Default,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Matrix3x3::default();
        for i in 0..3 {
            for j in 0..3 {
                res.m[i][j] = self.m[i][j] + rhs.m[i][j]
            }
        }
        res
    }
}

impl<F> Mul<F> for Matrix3x3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialEq
        + Default,
{
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        let mut res = Matrix3x3::default();
        for i in 0..3 {
            for j in 0..3 {
                res.m[i][j] = self.m[i][j] * rhs
            }
        }
        res
    }
}
impl<F> Mul for Matrix3x3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialEq
        + Default,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Matrix3x3::default();
        //i are the rows, j are the columns
        for i in 0..3 {
            for j in 0..3 {
                res.m[i][j] = self.m[i][0] * rhs.m[0][j]
                    + self.m[i][1] * rhs.m[1][j]
                    + self.m[i][2] * rhs.m[2][j]
            }
        }
        res
    }
}

impl<F> Sub for Matrix3x3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialEq
        + Default,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Matrix3x3::default();
        for i in 0..3 {
            for j in 0..3 {
                res.m[i][j] = self.m[i][j] - rhs.m[i][j]
            }
        }
        res
    }
}

impl<F> Matrix3x3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialEq
        + Default,
{
    pub fn unit_matrix() -> Self
    where
        F: From<u8>,
    {
        return Self {
            m: [
                [1.into(), 0.into(), 0.into()],
                [0.into(), 1.into(), 0.into()],
                [0.into(), 0.into(), 1.into()],
            ],
        };
    }

    pub fn determinant(&self) -> F {
        self.m[0][0] * self.m[1][1] * self.m[2][2]
            + self.m[1][0] * self.m[2][1] * self.m[0][2]
            + self.m[2][0] * self.m[0][1] * self.m[1][2]
            - self.m[0][2] * self.m[1][1] * self.m[2][0]
            - self.m[0][1] * self.m[1][0] * self.m[2][2]
            - self.m[0][0] * self.m[1][2] * self.m[2][1]
    }
    fn add_row(&self, i: usize, row: [F; 3]) -> Self {
        let mut t = self.m;
        t[i] = [t[i][0] + row[0], t[i][1] + row[1], t[i][2] + row[2]];
        Matrix3x3 { m: t }
    }

    fn mul_row(&self, i: usize, factor: F) -> [F; 3] {
        [
            self.m[i][0] * factor,
            self.m[i][1] * factor,
            self.m[i][2] * factor,
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn matrix_multiply() {
        let m1: Matrix3x3<f32> = Matrix3x3::unit_matrix();
        let m2: Matrix3x3<f32> = Matrix3x3 {
            m: [[3., 2., 5.], [2., -1., -3.], [4., -2., 3.]],
        };
        assert_eq!(m2, m1 * m2);
        assert_eq!(m2, m2 * m1);
    }
}
