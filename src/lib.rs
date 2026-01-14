pub mod hollow_polygon;
pub mod triangle;

fn signed_area(points: &[(f64, f64)]) -> f64 {
    assert!(points.len() >= 3);

    let mut sum = 0.0;

    for i in 0..points.len() {
        let p = points[i];
        let q = points[(i + 1) % points.len()];
        sum += p.0 * q.1 - q.0 * p.1;
    }

    0.5 * sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signed_area() {
        assert_eq!(signed_area(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]), 0.5);
        assert_eq!(signed_area(&[(0.0, 0.0), (0.0, 1.0), (1.0, 0.0)]), -0.5);

        assert_eq!(signed_area(&[(0.0, 0.0), (2.0, 0.0), (0.0, 2.0)]), 2.0);
        assert_eq!(signed_area(&[(0.0, 0.0), (0.0, 2.0), (2.0, 0.0)]), -2.0);

        assert_eq!(signed_area(&[(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0)]), 2.0);
        assert_eq!(signed_area(&[(-1.0, -1.0), (-1.0, 1.0), (1.0, -1.0)]), -2.0);

        assert_eq!(signed_area(&[(0.0, 0.0), (2.0, 0.0), (0.0, 4.0)]), 4.0);
        assert_eq!(signed_area(&[(0.0, 0.0), (0.0, 4.0), (2.0, 0.0)]), -4.0);
    }
}
