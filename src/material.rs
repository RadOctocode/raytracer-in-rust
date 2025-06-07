use crate::color::color;
use crate::color::Color;
use float_cmp::approx_eq;

#[derive(Debug, Clone)]
pub struct Material {
    pub ambient: f64,
    pub color: Color,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        let ambient_comp = approx_eq!(f64, self.ambient, other.ambient, epsilon = 0.0001);
        let diffuse_comp = approx_eq!(f64, self.diffuse, other.diffuse, epsilon = 0.0001);
        let specular_comp = approx_eq!(f64, self.specular, other.specular, epsilon = 0.0001);
        let shininess_comp = approx_eq!(f64, self.shininess, other.shininess, epsilon = 0.0001);
        let color_comp = self.color == other.color;
        return ambient_comp && diffuse_comp && specular_comp && shininess_comp && color_comp;
    }
}

impl Material {
    pub fn default_material() -> Material {
        Material {
            ambient: 0.1,
            color: color(1.0, 1.0, 1.0),
            diffuse: 0.9,
            specular: 0.9,
            shininess: 20.0,
        }
    }
}
