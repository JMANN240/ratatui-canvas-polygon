use std::convert::identity;

use itertools::Itertools;
use line_clipping::{LineSegment, Point, Window, cohen_sutherland::clip_line};
use ratatui_core::style::Color;
use ratatui_widgets::canvas::{Painter, Shape};

pub struct Triangle {
    pub coords: [(f64, f64); 3],
    pub color: Color,
}

impl Triangle {
    pub const fn new(coords: [(f64, f64); 3], color: Color) -> Self {
        Self { coords, color }
    }

    fn on_canvas_points(&self, bounds: (&[f64; 2], &[f64; 2])) -> Vec<(f64, f64)> {
        let maybe_line_1 = Self::clip_line(self.coords[0], self.coords[1], bounds);
        let maybe_line_2 = Self::clip_line(self.coords[1], self.coords[2], bounds);
        let maybe_line_3 = Self::clip_line(self.coords[2], self.coords[0], bounds);

        [maybe_line_1, maybe_line_2, maybe_line_3]
            .into_iter()
            .filter_map(identity)
            .flat_map(|line| [(line.p1.x, line.p1.y), (line.p2.x, line.p2.y)].into_iter())
            .unique_by(|(l, r)| (l.to_bits(), r.to_bits()))
            .collect()
    }

    fn bounding_box_canvas(&self, bounds: (&[f64; 2], &[f64; 2])) -> Option<(f64, f64, f64, f64)> {
        let maybe_line_1 = Self::clip_line(self.coords[0], self.coords[1], bounds);
        let maybe_line_2 = Self::clip_line(self.coords[1], self.coords[2], bounds);
        let maybe_line_3 = Self::clip_line(self.coords[2], self.coords[0], bounds);

        let maybe_line_1_x_min = maybe_line_1.map(|line| line.p1.x.min(line.p2.x));
        let maybe_line_1_x_max = maybe_line_1.map(|line| line.p1.x.max(line.p2.x));
        let maybe_line_1_y_min = maybe_line_1.map(|line| line.p1.y.min(line.p2.y));
        let maybe_line_1_y_max = maybe_line_1.map(|line| line.p1.y.max(line.p2.y));

        let maybe_line_2_x_min = maybe_line_2.map(|line| line.p1.x.min(line.p2.x));
        let maybe_line_2_x_max = maybe_line_2.map(|line| line.p1.x.max(line.p2.x));
        let maybe_line_2_y_min = maybe_line_2.map(|line| line.p1.y.min(line.p2.y));
        let maybe_line_2_y_max = maybe_line_2.map(|line| line.p1.y.max(line.p2.y));

        let maybe_line_3_x_min = maybe_line_3.map(|line| line.p1.x.min(line.p2.x));
        let maybe_line_3_x_max = maybe_line_3.map(|line| line.p1.x.max(line.p2.x));
        let maybe_line_3_y_min = maybe_line_3.map(|line| line.p1.y.min(line.p2.y));
        let maybe_line_3_y_max = maybe_line_3.map(|line| line.p1.y.max(line.p2.y));

        let maybe_x_min = [maybe_line_1_x_min, maybe_line_2_x_min, maybe_line_3_x_min]
            .into_iter()
            .filter_map(identity)
            .min_by(|l, r| l.partial_cmp(r).unwrap());

        let maybe_x_max = [maybe_line_1_x_max, maybe_line_2_x_max, maybe_line_3_x_max]
            .into_iter()
            .filter_map(identity)
            .max_by(|l, r| l.partial_cmp(r).unwrap());

        let maybe_y_min = [maybe_line_1_y_min, maybe_line_2_y_min, maybe_line_3_y_min]
            .into_iter()
            .filter_map(identity)
            .min_by(|l, r| l.partial_cmp(r).unwrap());

        let maybe_y_max = [maybe_line_1_y_max, maybe_line_2_y_max, maybe_line_3_y_max]
            .into_iter()
            .filter_map(identity)
            .max_by(|l, r| l.partial_cmp(r).unwrap());

        if let (Some(x_min), Some(x_max), Some(y_min), Some(y_max)) =
            (maybe_x_min, maybe_x_max, maybe_y_min, maybe_y_max)
        {
            Some((x_min, x_max, y_min, y_max))
        } else {
            None
        }
    }

    fn bounding_box_grid(&self, painter: &Painter) -> Option<(usize, usize, usize, usize)> {
        self.bounding_box_canvas(painter.bounds())
            .and_then(|bounds| {
                let maybe_p_min = painter.get_point(bounds.0, bounds.2);

                let Some(p_min) = maybe_p_min else {
                    return None;
                };

                let maybe_p_max = painter.get_point(bounds.1, bounds.3);

                let Some(p_max) = maybe_p_max else {
                    return None;
                };

                Some((p_min.0, p_max.0, p_max.1, p_min.1))
            })
    }

    fn clip_line(
        p1: (f64, f64),
        p2: (f64, f64),
        bounds: (&[f64; 2], &[f64; 2]),
    ) -> Option<LineSegment> {
        clip_line(
            LineSegment::new(Point::new(p1.0, p1.1), Point::new(p2.0, p2.1)),
            Window::new(bounds.0[0], bounds.0[1], bounds.1[0], bounds.1[1]),
        )
    }
}

impl Shape for Triangle {
    fn draw(&self, painter: &mut Painter) {
        let Some((x_min, x_max, y_min, y_max)) = self.bounding_box_grid(painter) else {
            return;
        };

        for x in x_min..x_max {
            for y in y_min..y_max {
                painter.paint(x, y, self.color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui_core::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};
    use ratatui_widgets::canvas::Canvas;

    #[test]
    fn test_on_canvas_points() {
        let triangle = Triangle::new([(0.0, 0.0), (2.0, 0.0), (0.0, 1.0)], Color::White);

        let zero_two_points = triangle.on_canvas_points((&[0.0, 2.0], &[0.0, 2.0]));
        assert_eq!(zero_two_points, &[(0.0, 0.0), (2.0, 0.0), (0.0, 1.0)]);

        let zero_one_points = triangle.on_canvas_points((&[0.0, 1.0], &[0.0, 1.0]));
        assert_eq!(zero_one_points, &[(0.0, 0.0), (1.0, 0.0), (1.0, 0.5), (0.0, 1.0)]);

        let six_seven_points = triangle.on_canvas_points((&[6.0, 7.0], &[6.0, 7.0]));
        assert_eq!(six_seven_points, &[]);
    }

    #[test]
    fn test_bounding_box_canvas_1() {
        let triangle = Triangle::new([(0.0, 0.0), (2.0, 0.0), (0.0, 1.0)], Color::White);

        let zero_two_bounds = triangle.bounding_box_canvas((&[0.0, 2.0], &[0.0, 2.0]));
        assert_eq!(zero_two_bounds, Some((0.0, 2.0, 0.0, 1.0)));

        let zero_one_bounds = triangle.bounding_box_canvas((&[0.0, 1.0], &[0.0, 1.0]));
        assert_eq!(zero_one_bounds, Some((0.0, 1.0, 0.0, 1.0)));

        let size_seven_bounds = triangle.bounding_box_canvas((&[6.0, 7.0], &[6.0, 7.0]));
        assert_eq!(size_seven_bounds, None);
    }

    #[test]
    fn test_bounding_box_canvas_2() {
        let triangle = Triangle::new([(0.25, 0.25), (0.25, 0.75), (0.75, 0.75)], Color::White);

        let bounds = triangle.bounding_box_canvas((&[0.0, 1.0], &[0.0, 1.0]));
        assert_eq!(bounds, Some((0.25, 0.75, 0.25, 0.75)));
    }

    #[test]
    fn test_render_triangle() {
        let canvas = Canvas::default()
            .x_bounds([0.0, 1.0])
            .y_bounds([0.0, 1.0])
            .paint(|context| {
                context.draw(&Triangle::new(
                    [(0.25, 0.25), (0.25, 0.75), (0.75, 0.75)],
                    Color::White,
                ));
            });

        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        canvas.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "                                                  ",
            "            ⢸⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⣉⣉⣉⡩⠭⠭⠭⠛⠛⠛⠋⠉⠁            ",
            "            ⢸⣀⡠⠤⠤⠤⠒⠒⠒⠊⠉⠉⠉                         ",
            "                                                  ",
        ]);

        let line_style = Style::default().white();

        expected.set_style(Rect::new(12, 1, 26, 1), line_style);
        expected.set_style(Rect::new(12, 2, 13, 1), line_style);

        assert_eq!(buf, expected);
    }
}
