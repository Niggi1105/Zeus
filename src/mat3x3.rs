use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Matrix3x3<F>
where
    F: Mul<Output = F>
        + Div<Output = F>
        + Add<Output = F>
        + Sub<Output = F>
        + Copy
        + PartialEq
        + Default
        + Neg<Output = F>,
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
        + Default
        + Neg<Output = F>,
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
        + Default
        + Neg<Output = F>,
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
        + Default
        + Neg<Output = F>,
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
        + Default
        + Neg<Output = F>,
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
        + Default
        + Neg<Output = F>,
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

    fn add_row(&mut self, i: usize, row: [F; 3]) {
        self.m[i] = [
            self.m[i][0] + row[0],
            self.m[i][1] + row[1],
            self.m[i][2] + row[2],
        ];
    }

    fn mul_row(&self, i: usize, factor: F) -> [F; 3] {
        [
            self.m[i][0] * factor,
            self.m[i][1] * factor,
            self.m[i][2] * factor,
        ]
    }

    //this is cancer, don't try to understand it, you will fail!
    fn solve(&self, mut b: [F; 3]) -> Option<[F; 3]>
    where
        F: From<u8>,
    {
        if self.determinant() == 0.into() {
            return None;
        }
        let mut tmp = self.to_owned();

        //create zeros in the first column
        //second row
        let mut k = -(tmp.m[1][0] / tmp.m[0][0]);
        tmp.add_row(1, tmp.mul_row(0, k));
        b[1] = b[1] + k * b[0];

        //third row
        k = -(tmp.m[2][0] / tmp.m[0][0]);
        tmp.add_row(2, tmp.mul_row(0, k));
        b[2] = b[2] + k * b[0];

        //create zero in second column
        k = -(tmp.m[2][1] / tmp.m[1][1]);
        tmp.add_row(2, tmp.mul_row(1, k));
        b[2] = b[2] + k * b[1];

        //create the 1 in bottom right
        b[2] = b[2] / tmp.m[2][2];
        tmp.m[2][2] = tmp.m[2][2] / tmp.m[2][2];

        //zeros in 3rd column
        b[1] = b[1] - tmp.m[1][2] * b[2];
        tmp.add_row(1, tmp.mul_row(2, -tmp.m[1][2]));

        b[0] = b[0] - tmp.m[0][2] * b[2];
        tmp.add_row(0, tmp.mul_row(2, -tmp.m[0][2]));

        //create 1 in center
        b[1] = b[1] / tmp.m[1][1];
        tmp.m[1][1] = tmp.m[1][1] / tmp.m[1][1];

        //zero in second column
        b[0] = b[0] - tmp.m[0][1] * b[1];
        tmp.add_row(0, tmp.mul_row(1, -tmp.m[0][1]));

        Some(b)
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

    #[test]
    fn solve_lin_eq() {
        let m2: Matrix3x3<f32> = Matrix3x3 {
            m: [[1., 1., 1.], [0., 1., 1.], [1., 0., -1.]],
        };
        let v = [2., 2., 2.];
        let s = [0., 4., -2.];
        assert_eq!(s, m2.solve(v).unwrap());
    }

    #[test]
    fn solve_lin_eq_inf() {
        let m2: Matrix3x3<f32> = Matrix3x3 {
            m: [[1., 1., 1.], [0., 1., 1.], [0., 0., 0.]],
        };
        let v = [2., 2., 2.];
        assert_eq!(None, m2.solve(v));
    }
}
