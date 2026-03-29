const BULLSEYE_RADIUS_SQ: f32 = 1.0; // 1² = 1
const INNER_RING_RADIUS_SQ: f32 = 25.0; // 5² = 25
const OUTER_RING_RADIUS_SQ: f32 = 100.0; // 10² = 100

#[inline]
pub fn calculate_score(x: f32, y: f32) -> Option<i8> {
    if x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite() {
        return None;
    }
    let r_sq = x * x + y * y;
    if r_sq < BULLSEYE_RADIUS_SQ {
        Some(10)
    } else if r_sq < INNER_RING_RADIUS_SQ {
        Some(5)
    } else if r_sq < OUTER_RING_RADIUS_SQ {
        Some(1)
    } else {
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_10() {
        assert_eq!(calculate_score(0.0, 0.0), Some(10));
        assert_eq!(calculate_score(0.7, 0.7), Some(10));
        assert_eq!(calculate_score(-0.7, 0.7), Some(10));
        assert_eq!(calculate_score(0.7, -0.7), Some(10));
        assert_eq!(calculate_score(-0.7, -0.7), Some(10));
    }

    #[test]
    fn test_score_5() {
        assert_eq!(calculate_score(1.4, 1.4), Some(5));
        assert_eq!(calculate_score(-1.4, 1.4), Some(5));
        assert_eq!(calculate_score(1.4, -1.4), Some(5));
        assert_eq!(calculate_score(-1.4, -1.4), Some(5));
    }

    #[test]
    fn test_score_1() {
        assert_eq!(calculate_score(4.0, 4.0), Some(1));
        assert_eq!(calculate_score(-4.0, 4.0), Some(1));
        assert_eq!(calculate_score(4.0, -4.0), Some(1));
        assert_eq!(calculate_score(-4.0, -4.0), Some(1));
    }

    #[test]
    fn test_score_0() {
        assert_eq!(calculate_score(11.0, 11.0), Some(0));
        assert_eq!(calculate_score(-11.0, 11.0), Some(0));
        assert_eq!(calculate_score(11.0, -11.0), Some(0));
        assert_eq!(calculate_score(-11.0, -11.0), Some(0));
    }

    #[test]
    fn test_invalid_nan() {
        assert_eq!(calculate_score(f32::NAN, 0.0), None);
        assert_eq!(calculate_score(0.0, f32::NAN), None);
        assert_eq!(calculate_score(f32::NAN, f32::NAN), None);
    }

    #[test]
    fn test_invalid_infinity() {
        assert_eq!(calculate_score(f32::INFINITY, 0.0), None);
        assert_eq!(calculate_score(0.0, f32::NEG_INFINITY), None);
        assert_eq!(calculate_score(f32::INFINITY, f32::NEG_INFINITY), None);
    }
}
