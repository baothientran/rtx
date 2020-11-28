pub const EPSILON_F32_1: f32 = 0.1;
pub const EPSILON_F32_2: f32 = 0.01;
pub const EPSILON_F32_3: f32 = 0.001;
pub const EPSILON_F32_4: f32 = 0.0001;
pub const EPSILON_F32_5: f32 = 0.00001;
pub const EPSILON_F32_6: f32 = 0.000001;

pub const EPSILON_F64_1: f64 = 0.1;
pub const EPSILON_F64_2: f64 = 0.01;
pub const EPSILON_F64_3: f64 = 0.001;
pub const EPSILON_F64_4: f64 = 0.0001;
pub const EPSILON_F64_5: f64 = 0.00001;
pub const EPSILON_F64_6: f64 = 0.000001;
pub const EPSILON_F64_7: f64 = 0.0000001;
pub const EPSILON_F64_8: f64 = 0.00000001;
pub const EPSILON_F64_9: f64 = 0.000000001;
pub const EPSILON_F64_10: f64 = 0.0000000001;
pub const EPSILON_F64_11: f64 = 0.00000000001;
pub const EPSILON_F64_12: f64 = 0.000000000001;
pub const EPSILON_F64_13: f64 = 0.0000000000001;
pub const EPSILON_F64_14: f64 = 0.00000000000001;

pub fn equal_epsilon_f32(lhs: f32, rhs: f32, epsilon: f32) -> bool {
    return (lhs - rhs).abs() < epsilon;
}

pub fn equal_epsilon_f64(lhs: f64, rhs: f64, epsilon: f64) -> bool {
    return (lhs - rhs).abs() < epsilon;
}

pub fn clamp<T>(num: T, min: T, max: T) -> T where T: PartialOrd {
    if min > max {
        panic!("min should smaller than max");
    }

    if num < min {
        return min;
    }

    if num > max {
        return max;
    }

    return num;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_equal_epsilon_f32() {
        assert!(equal_epsilon_f32(5.1, 5.100001, EPSILON_F32_6));
        assert!(!equal_epsilon_f32(5.100004, 5.100001, EPSILON_F32_6));
    }

    #[test]
    fn test_equal_epsilon_f64() {
        assert!(equal_epsilon_f64(5.1, 5.10000000000001, EPSILON_F64_13));
        assert!(!equal_epsilon_f64(
            5.10000000000001,
            5.100001,
            EPSILON_F64_13
        ));
    }

    #[test]
    fn test_clamp() {
        let n: i8 = 35;
        let i = clamp(n, 34, 40);
        assert_eq!(i, 35);
    }

    #[test]
    fn test_clamp_max() {
        let n: i8 = 50;
        let i = clamp(n, 34, 40);
        assert_eq!(i, 40);
    }

    #[test]
    fn test_clamp_min() {
        let n: i8 = 10;
        let i = clamp(n, 34, 40);
        assert_eq!(i, 34);
    }
}
