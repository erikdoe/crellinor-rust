use std::ops::Mul;

pub fn round(val: f64, p: i32) -> f64 {
    let f = 10.0_f64.powi(p);
    ((val * f).round())/f
}

pub fn square<T>(x: T) -> T
    where T: Copy + Mul<Output = T>  {
    x * x
}