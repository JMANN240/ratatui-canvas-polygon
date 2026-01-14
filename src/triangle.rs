use itertools::Itertools;
use line_clipping::{LineSegment, Point, Window, cohen_sutherland::clip_line};
use ratatui_core::style::Color;
use ratatui_widgets::canvas::{Painter, Shape};

use crate::signed_area;

pub struct Triangle {
    pub coords: [(f64, f64); 3],
    pub color: Color,
}

impl Triangle {
    pub fn new(coords: [(f64, f64); 3], color: Color) -> Self {
        let mut first = Self { coords, color };

        if !first.is_wound_ccw() {
            first.coords.reverse();
        }

        first
    }

    fn signed_area(&self) -> f64 {
        signed_area(&self.coords)
    }

    fn is_wound_ccw(&self) -> bool {
        self.signed_area().is_sign_positive()
    }

    fn on_canvas_points(&self, bounds: (&[f64; 2], &[f64; 2])) -> Vec<(f64, f64)> {
        let maybe_line_1 = Self::clip_line(self.coords[0], self.coords[1], bounds);
        let maybe_line_2 = Self::clip_line(self.coords[1], self.coords[2], bounds);
        let maybe_line_3 = Self::clip_line(self.coords[2], self.coords[0], bounds);

        [maybe_line_1, maybe_line_2, maybe_line_3]
            .into_iter()
            .flatten()
            .flat_map(|line| [(line.p1.x, line.p1.y), (line.p2.x, line.p2.y)].into_iter())
            .unique_by(|(l, r)| (l.to_bits(), r.to_bits()))
            .collect()
    }

    fn on_grid_points(&self, painter: &Painter) -> Vec<(usize, usize)> {
        self.on_canvas_points(painter.bounds())
            .iter()
            .filter_map(|canvas_point| painter.get_point(canvas_point.0, canvas_point.1))
            .collect()
    }

    fn on_grid_edges(&self, painter: &Painter) -> Vec<((usize, usize), (usize, usize))> {
        let points = self.on_grid_points(painter);
        let mut rotated_points = points.clone();
        rotated_points.rotate_right(1);
        points.into_iter().zip(rotated_points).collect()
    }

    fn grid_point_in_self(&self, painter: &Painter, grid_point: (usize, usize)) -> bool {
        self.on_grid_edges(painter)
            .into_iter()
            .map(|(p1, p2)| {
                let area = signed_area(&[
                    (p1.0 as f64, p1.1 as f64),
                    (p2.0 as f64, p2.1 as f64),
                    (grid_point.0 as f64, grid_point.1 as f64),
                ]);

                area.is_sign_positive()
            })
            .all_equal()
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
            .flatten()
            .min_by(|l, r| l.partial_cmp(r).unwrap());

        let maybe_x_max = [maybe_line_1_x_max, maybe_line_2_x_max, maybe_line_3_x_max]
            .into_iter()
            .flatten()
            .max_by(|l, r| l.partial_cmp(r).unwrap());

        let maybe_y_min = [maybe_line_1_y_min, maybe_line_2_y_min, maybe_line_3_y_min]
            .into_iter()
            .flatten()
            .min_by(|l, r| l.partial_cmp(r).unwrap());

        let maybe_y_max = [maybe_line_1_y_max, maybe_line_2_y_max, maybe_line_3_y_max]
            .into_iter()
            .flatten()
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
                let p_min = painter.get_point(bounds.0, bounds.2)?;
                let p_max = painter.get_point(bounds.1, bounds.3)?;

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
                if self.grid_point_in_self(painter, (x, y)) {
                    painter.paint(x, y, self.color);
                }
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
        assert_eq!(
            zero_one_points,
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 0.5), (0.0, 1.0)]
        );

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
                    [(0.25, 0.25), (0.25, 0.75), (0.5, 0.75)],
                    Color::White,
                ));
                context.draw(&Triangle::new(
                    [(0.75, 0.25), (0.75, 0.75), (0.5, 0.75)],
                    Color::White,
                ));
            });

        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        canvas.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "                                                  ",
            "            ⢸⣿⣿⣿⣿⣿⣿⣿⠿⠟⠛⠋⠉⠉⠉⠛⠻⠿⢿⣿⣿⣿⣿⣿⣿             ",
            "            ⠸⠿⠟⠛⠋⠉              ⠉⠉⠛⠻⠿             ",
            "                                                  ",
        ]);

        let line_style = Style::default().white();

        expected.set_style(Rect::new(12, 1, 25, 1), line_style);
        expected.set_style(Rect::new(12, 2, 6, 1), line_style);
        expected.set_style(Rect::new(32, 2, 5, 1), line_style);

        assert_eq!(buf, expected);
    }
}
