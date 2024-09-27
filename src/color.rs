#![allow(dead_code)]
use float_cmp::approx_eq;
#[derive(Debug)]
struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let red_comp = approx_eq!(f64, self.red, other.red, ulps = 2);
        let green_comp = approx_eq!(f64, self.green, other.green, ulps = 2);
        let blue_comp = approx_eq!(f64, self.blue, other.blue, ulps = 2);
        return red_comp && green_comp && blue_comp;
    }
}

fn color(_red: f64, _green: f64, _blue: f64) -> Color {
    return Color {
        red: _red,
        green: _green,
        blue: _blue,
    };
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, _rhs: Color) -> Color {
        return color(
            self.red + _rhs.red,
            self.green + _rhs.green,
            self.blue + _rhs.blue,
        );
    }
}

impl std::ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, _rhs: Color) -> Color {
        return color(
            self.red - _rhs.red,
            self.green - _rhs.green,
            self.blue - _rhs.blue,
        );
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        return color(
            self.red * _rhs.red,
            self.green * _rhs.green,
            self.blue * _rhs.blue,
        );
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, _rhs: f64) -> Color {
        return color(self.red * _rhs, self.green * _rhs, self.blue * _rhs);
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        return color(self * _rhs.red, self * _rhs.green, self * _rhs.blue);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_color_constructor() {
        let c = color(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn test_adding_colors() {
        let a = color(0.9, 0.6, 0.75);
        let b = color(0.7, 0.1, 0.25);
        let expected = color(1.6, 0.7, 1.0);
        let actual = a + b;
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_subtracting_colors() {
        let a = color(0.9, 0.6, 0.75);
        let b = color(0.7, 0.1, 0.25);
        let expected = color(0.2, 0.5, 0.5);
        let actual = a - b;
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_multiplying_colors_by_constants() {
        let a = color(0.2, 0.3, 0.4);
        let expected = color(0.4, 0.6, 0.8);
        let actual = a * 2.0;
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_multiplying_colors_by_colors() {
        let a = color(1.0, 0.2, 0.4);
        let b = color(0.9, 1.0, 0.1);
        let expected = color(0.9, 0.2, 0.04);
        let actual = a * b;
        assert_eq!(actual, expected);
    }
}
