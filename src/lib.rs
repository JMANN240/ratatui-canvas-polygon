use ratatui_core::style::Color;
use ratatui_widgets::canvas::{Line, Painter, Shape};

pub struct HollowPolygon<'a> {
    pub coords: &'a [(f64, f64)],
    pub color: Color,
}

impl<'a> HollowPolygon<'a> {
    pub const fn new(coords: &'a [(f64, f64)], color: Color) -> Self {
        Self { coords, color }
    }
}

impl Shape for HollowPolygon<'_> {
    fn draw(&self, painter: &mut Painter) {
        if let Some(first_coord) = self.coords.first() && let Some(last_coord) = self.coords.last() {
            for ((x1, y1), (x2, y2)) in self.coords.iter().zip(self.coords.iter().skip(1)) {
                Line::new(*x1, *y1, *x2, *y2, self.color).draw(painter);
            }

            Line::new(last_coord.0, last_coord.1, first_coord.0, first_coord.1, self.color).draw(painter)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui_core::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};
    use ratatui_widgets::canvas::Canvas;

    #[test]
    fn render_1() {
        let canvas = Canvas::default()
            .x_bounds([0.0, 1.0])
            .y_bounds([0.0, 1.0])
            .paint(|context| {
                context.draw(&HollowPolygon::new(&[(0.25, 0.25)], Color::White));
            });

        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        canvas.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "                                                  ",                                                                                                                                           
            "                                                  ",                                                                                                                                           
            "            ⢀                                     ",                                                                                                                                           
            "                                                  ",
        ]);

        let line_style = Style::default().white();

        expected.set_style(Rect::new(12,2, 1, 1), line_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn render_2() {
        let canvas = Canvas::default()
            .x_bounds([0.0, 1.0])
            .y_bounds([0.0, 1.0])
            .paint(|context| {
                context.draw(&HollowPolygon::new(&[(0.25, 0.25),(0.75, 0.75)], Color::White));
            });

        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        canvas.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "                                                  ",                                                                                                                                           
            "                         ⣀⣀⣀⡠⠤⠤⠤⠒⠒⠒⠊⠉⠁            ",                                                                                                                                           
            "            ⢀⣀⡠⠤⠤⠤⠒⠒⠒⠊⠉⠉⠉                         ",                                                                                                                                           
            "                                                  ",
        ]);

        let line_style = Style::default().white();

        expected.set_style(Rect::new(25, 1, 13, 1), line_style);
        expected.set_style(Rect::new(12,2, 13, 1), line_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn render_3() {
        let canvas = Canvas::default()
            .x_bounds([0.0, 1.0])
            .y_bounds([0.0, 1.0])
            .paint(|context| {
                context.draw(&HollowPolygon::new(&[(0.25, 0.25), (0.25, 0.75), (0.75, 0.75)], Color::White));
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
        expected.set_style(Rect::new(12,2, 13, 1), line_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn render_4() {
        let canvas = Canvas::default()
            .x_bounds([0.0, 1.0])
            .y_bounds([0.0, 1.0])
            .paint(|context| {
                context.draw(&HollowPolygon::new(&[(0.25, 0.5), (0.5, 0.75), (0.75, 0.5), (0.5, 0.25)], Color::White));
            });

        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        canvas.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "                                                  ",                                                                                                                                           
            "              ⢀⣀⣀⡠⠤⠤⠔⠒⠒⠊⠉⠉⠉⠒⠒⠒⠤⠤⠤⣀⣀⣀              ",                                                                                                                                           
            "            ⠈⠉⠉⠒⠒⠒⠒⠤⠤⠤⠤⣀⣀⣀⣀⡠⠤⠤⠤⠔⠒⠒⠒⠊⠉⠁            ",                                                                                                                                           
            "                                                  ",
        ]);

        let line_style = Style::default().white();

        expected.set_style(Rect::new(14, 1, 22, 1), line_style);
        expected.set_style(Rect::new(12,2, 26, 1), line_style);

        assert_eq!(buf, expected);
    }
}
