pub mod hollow_polygon;
pub mod triangle;

fn signed_triangle_area(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
    0.5 * ((b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signed_triangle_area() {
        assert_eq!(signed_triangle_area((0.0, 0.0), (1.0, 0.0), (0.0, 1.0)), 0.5);
        assert_eq!(signed_triangle_area((0.0, 0.0), (0.0, 1.0), (1.0, 0.0)), -0.5);

        assert_eq!(signed_triangle_area((0.0, 0.0), (2.0, 0.0), (0.0, 2.0)), 2.0);
        assert_eq!(signed_triangle_area((0.0, 0.0), (0.0, 2.0), (2.0, 0.0)), -2.0);

        assert_eq!(signed_triangle_area((-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0)), 2.0);
        assert_eq!(signed_triangle_area((-1.0, -1.0), (-1.0, 1.0), (1.0, -1.0)), -2.0);

        assert_eq!(signed_triangle_area((0.0, 0.0), (2.0, 0.0), (0.0, 4.0)), 4.0);
        assert_eq!(signed_triangle_area((0.0, 0.0), (0.0, 4.0), (2.0, 0.0)), -4.0);
    }
}
