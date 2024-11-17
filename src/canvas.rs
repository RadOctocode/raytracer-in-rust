#![allow(dead_code)]
use crate::color::color;
use crate::color::Color;
use float_cmp::approx_eq;
use std::fs::File;
use std::io::prelude;
use std::io::Write;

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    vector: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn zero(width: usize, height: usize) -> Self {
        let vector = vec![vec![color(0.0, 0.0, 0.0); width]; height];
        Canvas {
            width: width,
            height: height,
            vector: vector,
        }
    }
    pub fn set(vector: Vec<Vec<Color>>) -> Self {
        let mut width = 0;
        if vector.len() > 0 {
            width = vector[0].len();
        }
        Canvas {
            width: width,
            height: vector.len(),
            vector: vector,
        }
    }

    pub fn write_pixel(&mut self, row: usize, col: usize, current_color: Color) {
        self.vector[row][col] = current_color;
    }

    pub fn read_pixel(&self, row: usize, col: usize) -> Color {
        self.vector[row][col].clone()
    }

    fn scale_color(color: Color) -> (u8, u8, u8) {
        (
            Self::scale_component(color.red),
            Self::scale_component(color.blue),
            Self::scale_component(color.green),
        )
    }

    fn scale_component(component: f64) -> u8 {
        let mut scale_value = component;
        if component > 1.00 {
            scale_value = 1.00;
        }

        if component < 0.00 {
            scale_value = 0.00;
        }
        return (255.00 * scale_value) as u8;
    }

    pub fn canvas_to_ppm(&self, name: &str) -> std::io::Result<()> {
        let mut file = File::options()
            .append(true)
            .create_new(true)
            .open(name)
            .unwrap();
        writeln!(
            &mut file,
            "P3\n{width} {height}\n255",
            width = self.width,
            height = self.height
        );
        for element in &self.vector {
            for color in element {
                let (red, blue, green) = Self::scale_color(color.clone());
                println!("{} {} {}\n", red, blue, green);
                writeln!(&mut file, "{} {} {}\n", red, blue, green);
            }
        }
        writeln!(&mut file, "\n");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_pixel() {
        let mut canvas = Canvas::zero(5, 3);
        let red = color(1.00, 0.00, 0.00);
        canvas.write_pixel(0, 0, red.clone());
        assert_eq!(red, canvas.read_pixel(0, 0));
    }
}
