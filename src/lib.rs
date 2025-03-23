// round - rust library (crate)
// GNU licensed, license file can be found at the root of the repository
// Mohamed Hayibor - Copyright 2016

use num_traits::Float;

pub fn round<T: Float>(number: T, rounding: i32) -> T {
    let scale = T::from(10).expect("Can create").powi(rounding);
    (number * scale).round() / scale
}

// implementing round_up and round_down with same design pattern
pub fn round_up<T: Float>(number: T, rounding: i32) -> T {
    let scale = T::from(10).expect("Can create").powi(rounding);
    (number * scale).ceil() / scale
}

pub fn round_down<T: Float>(number: T, rounding: i32) -> T {
    let scale = T::from(10).expect("Can create").powi(rounding);
    (number * scale).floor() / scale
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        let pi: f64 = std::f64::consts::PI;

        assert_eq!(round(pi, 0), 3.0);
        assert_eq!(round(pi, 1), 3.1);
        assert_eq!(round(pi, 2), 3.14);
        assert_eq!(round(pi, 3), 3.142);
        assert_eq!(round(pi, 4), 3.1416);
        assert_eq!(round(pi, 5), 3.14159);
        assert_eq!(round(pi, 6), 3.141593);
        assert_eq!(round(pi, 7), 3.1415927);
        assert_eq!(round(pi, 8), 3.14159265);
    }

    #[test]
    fn test_round_down() {
        let pi: f64 = std::f64::consts::PI;

        assert_eq!(round_down(pi, 0), 3.0);
        assert_eq!(round_down(pi, 1), 3.1);
        assert_eq!(round_down(pi, 2), 3.14);
        assert_eq!(round_down(pi, 3), 3.141);
        assert_eq!(round_down(pi, 4), 3.1415);
        assert_eq!(round_down(pi, 5), 3.14159);
        assert_eq!(round_down(pi, 6), 3.141592);
        assert_eq!(round_down(pi, 7), 3.1415926);
        assert_eq!(round_down(pi, 8), 3.14159265);
    }

    #[test]
    fn test_round_up() {
        let pi: f64 = std::f64::consts::PI;

        assert_eq!(round_up(pi, 0), 4.0);
        assert_eq!(round_up(pi, 1), 3.2);
        assert_eq!(round_up(pi, 2), 3.15);
        assert_eq!(round_up(pi, 3), 3.142);
        assert_eq!(round_up(pi, 4), 3.1416);
        assert_eq!(round_up(pi, 5), 3.14160);
        assert_eq!(round_up(pi, 6), 3.141593);
        assert_eq!(round_up(pi, 7), 3.1415927);
        assert_eq!(round_up(pi, 8), 3.14159266);
    }
}
