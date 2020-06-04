use std::vec::Vec;
use crate::color::Color;

struct Canvas {
    pixels: Vec<Vec<Color>>
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        Canvas {
            pixels: vec![vec![Color { r: 0.0, g: 0.0, b: 0.0 }; height]; width]
        }
    }

    fn width(&self) -> usize {
        self.pixels.len()
    }

    fn height(&self) -> usize {
        if self.pixels.is_empty() { 0 } else { self.pixels[0].len() }
    }

    fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[x][y]
    }

    fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[x][y] = c
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::color::Color;
    use crate::color::test_utils::assert_color_eq;

    #[test]
    fn create_canvas() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(10, canvas.width());
        assert_eq!(20, canvas.height());
        for x in 0..canvas.width() {
            for y in 0..canvas.height() {
                assert_color_eq(Color { r: 0.0, g: 0.0, b: 0.0 }, canvas.get_pixel(x, y));
            }
        }
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let color = Color { r: 1.0, g: 0.0, b: 0.0 };

        canvas.set_pixel(2, 3, color);

        assert_color_eq(color, canvas.get_pixel(2, 3));
    }
}