use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Complex<F>
where
    F: Add + Mul + Div + Sub + PartialOrd + Copy,
{
    re: F,
    im: F,
}

impl<F> Mul for Complex<F>
where
    F: Add<Output = F> + Mul<Output = F> + Div<Output = F> + Sub<Output = F> + Ord + Copy,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl<F> Mul<F> for Complex<F>
where
    F: Add<Output = F> + Mul<Output = F> + Div<Output = F> + Sub<Output = F> + PartialOrd + Copy,
{
    type Output = Self;
    fn mul(self, rhs: F) -> Self::Output {
        Complex {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}
impl<F> Div<F> for Complex<F>
where
    F: Add<Output = F> + Mul<Output = F> + Div<Output = F> + Sub<Output = F> + PartialOrd + Copy,
{
    type Output = Self;
    fn div(self, rhs: F) -> Self::Output {
        Complex {
            re: self.re / rhs,
            im: self.re / rhs,
        }
    }
}
impl<F> Div for Complex<F>
where
    F: Add<Output = F> + Mul<Output = F> + Div<Output = F> + Sub<Output = F> + PartialOrd + Copy,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let d = rhs.re * rhs.re - rhs.im * rhs.im;
        Complex {
            re: (rhs.re * self.re - rhs.im * self.im) / d,
            im: (rhs.re * self.im - self.re * rhs.im) / d,
        }
    }
}

impl<F> Add for Complex<F>
where
    F: Add<Output = F> + Mul<Output = F> + Div<Output = F> + Sub<Output = F> + PartialOrd + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<F> Sub for Complex<F>
where
    F: Add<Output = F> + Mul<Output = F> + Div<Output = F> + Sub<Output = F> + PartialOrd + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication() {
        let c = Complex { re: 3, im: 1 };
        let r = c * c;
        assert_eq!(r.re, 8);
        assert_eq!(r.im, 6);
    }
    #[test]
    fn division() {
        let c = Complex { re: 3, im: 1 };
        let r = c / c;
        assert_eq!(r.re, 1);
        assert_eq!(r.im, 0);
    }
    #[test]
    fn addition() {
        let c1 = Complex { re: 3., im: 1. };
        let c2 = Complex { re: 2., im: 4. };
        let r = c1 + c2;
        assert_eq!(r.re, 5.);
        assert_eq!(r.im, 5.);
    }
    #[test]
    fn scalar_mul() {
        let c = Complex { re: 3., im: 1. };
        let r = c * 8.;
        assert_eq!(r.re, 24.);
        assert_eq!(r.im, 8.);
    }
}
