use crate::color::Color;
use std::vec::Vec;

pub struct Canvas {
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            pixels: vec![
                vec![
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0
                    };
                    height
                ];
                width
            ],
        }
    }

    pub fn width(&self) -> usize {
        self.pixels.len()
    }

    pub fn height(&self) -> usize {
        if self.pixels.is_empty() {
            0
        } else {
            self.pixels[0].len()
        }
    }

    // x: column, y: row
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[x][y]
    }

    // x: column, y: row
    pub fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[x][y] = c
    }

    fn to_ppm(&self) -> String {
        let mut result = format!(
            "P3\n{width} {height}\n255\n",
            width = self.width(),
            height = self.height(),
        );
        for y in 0..self.height() {
            let mut row_str = Vec::new();
            for x in 0..self.width() {
                let p = self.get_pixel(x, y);
                let r = self.convert(p.r);
                let g = self.convert(p.g);
                let b = self.convert(p.b);
                row_str.push(format!("{} {} {}", r, g, b));
            }
            result.push_str(&row_str.join(" "));
            result.push_str("\n");
        }

        result
    }

    pub fn write_ppm(&self, file: String) {
        use std::fs;
        let data = self.to_ppm();
        fs::write(file, data).expect("Unable to write file");
    }

    fn convert(&self, val: f64) -> u8 {
        self.clamp(val * 255., 0., 255.).round() as u8
    }

    fn clamp(&self, val: f64, min: f64, max: f64) -> f64 {
        if val < min {
            min
        } else if val > max {
            max
        } else {
            val
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::canvas::Canvas;
    use crate::color::Color;
    use crate::test_utils::assert_color_eq;

    #[test]
    fn create_canvas() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(10, canvas.width());
        assert_eq!(20, canvas.height());
        for x in 0..canvas.width() {
            for y in 0..canvas.height() {
                assert_color_eq(
                    Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                    },
                    canvas.get_pixel(x, y),
                );
            }
        }
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let color = Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        };

        canvas.set_pixel(2, 3, color);

        assert_color_eq(color, canvas.get_pixel(2, 3));
    }

    #[test]
    fn to_ppm_constructs_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let ppm_str = canvas.to_ppm();
        let header_lines: Vec<&str> = ppm_str.lines().take(3).collect();
        assert_eq!("P3", header_lines[0]);
        assert_eq!("5 3", header_lines[1]);
        assert_eq!("255", header_lines[2]);
    }

    #[test]
    fn to_ppm_constructs_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        canvas.set_pixel(0, 0, c1);
        canvas.set_pixel(2, 1, c2);
        canvas.set_pixel(4, 2, c3);

        let ppm_str = canvas.to_ppm();
        let pixel_data: Vec<&str> = ppm_str.lines().skip(3).take(3).collect();
        assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", pixel_data[0]);
        assert_eq!("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", pixel_data[1]);
        assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", pixel_data[2]);
    }

    #[test]
    fn to_ppm_adds_terminating_newline() {
        let canvas = Canvas::new(5, 3);
        let ppm_str = canvas.to_ppm();
        assert_eq!('\n', ppm_str.chars().last().unwrap());
    }
}
