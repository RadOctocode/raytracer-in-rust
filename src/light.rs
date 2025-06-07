use crate::color::color;
use crate::color::Color;
use crate::material::Material;
use crate::ray::reflect;
use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct PointLight {
    intensity: Color,
    position: Tuple,
}

impl PointLight {
    pub fn set_point_light(intensity: Color, position: Tuple) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }
}

pub fn lighting(
    m: Material,
    light: PointLight,
    point: Tuple,
    eye_vector: Tuple,
    normal_vector: Tuple,
) -> Color {
    let diffuse: Color;
    let specular: Color;
    let effective_color = m.color * light.intensity.clone();

    let light_vector = (light.position - point).normalize();
    let ambient = effective_color.clone() * m.ambient;
    let light_dot_normal = Tuple::dot(light_vector.clone(), normal_vector.clone());
    if light_dot_normal < 0.0 {
        diffuse = Color::create_black();
        specular = Color::create_black();
    } else {
        diffuse = effective_color.clone() * m.diffuse * light_dot_normal;

        let reflect_vector = reflect(-light_vector, normal_vector);
        let reflect_dot_eye = Tuple::dot(reflect_vector.clone(), eye_vector.clone());

        if reflect_dot_eye <= 0.0 {
            specular = Color::create_black()
        } else {
            let factor = f64::powf(reflect_dot_eye, m.shininess);
            specular = light.intensity.clone() * m.specular * factor;
        }
    }

    ambient + diffuse + specular
    //specular is weirdly not working
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_struct() {
        let intensity = color(1.0, 1.0, 1.0);
        let position = Tuple::set_point(0.0, 0.0, 0.0);
        let point_light = PointLight::set_point_light(intensity.clone(), position.clone());
        assert_eq!(point_light.intensity, intensity);
        assert_eq!(point_light.position, position);
    }

    #[test]
    fn test_lighting_flat() {
        let m = Material::default_material();
        let position = Tuple::set_point(0.0, 0.0, 0.0);

        let eye_vector = Tuple::set_vector(0.0, 0.0, -1.0);
        let normal_vector = Tuple::set_vector(0.0, 0.0, -1.0);

        let light_intensity = color(1.0, 1.0, 1.0);
        let light_position = Tuple::set_point(0.0, 0.0, -10.0);
        let point_light = PointLight::set_point_light(light_intensity, light_position);
        let actual = lighting(m, point_light, position, eye_vector, normal_vector);

        let expected = color(1.9, 1.9, 1.9);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lighting_eye_45_offset() {
        let m = Material::default_material();
        let position = Tuple::set_point(0.0, 0.0, 0.0);

        let eye_vector = Tuple::set_vector(0.0, (2.0_f64.sqrt() / 2.0), (-2.0_f64.sqrt() / 2.0));
        let normal_vector = Tuple::set_vector(0.0, 0.0, -1.0);

        let light_intensity = color(1.0, 1.0, 1.0);
        let light_position = Tuple::set_point(0.0, 0.0, -10.0);
        let point_light = PointLight::set_point_light(light_intensity, light_position);
        let actual = lighting(m, point_light, position, eye_vector, normal_vector);

        let expected = color(1.0, 1.0, 1.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lighting_light_45_offset() {
        let m = Material::default_material();
        let position = Tuple::set_point(0.0, 0.0, 0.0);

        let eye_vector = Tuple::set_vector(0.0, 0.0, -1.0);
        let normal_vector = Tuple::set_vector(0.0, 0.0, -1.0);

        let light_intensity = color(1.0, 1.0, 1.0);
        let light_position = Tuple::set_point(0.0, 10.0, -10.0);
        let point_light = PointLight::set_point_light(light_intensity, light_position);
        let actual = lighting(m, point_light, position, eye_vector, normal_vector);

        let expected = color(0.7364, 0.7364, 0.7364);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lighting_eye_reflection_path() {
        let m = Material::default_material();
        let position = Tuple::set_point(0.0, 0.0, 0.0);

        let eye_vector = Tuple::set_vector(0.0, (-2.0_f64.sqrt() / 2.0), (-2.0_f64.sqrt() / 2.0));
        let normal_vector = Tuple::set_vector(0.0, 0.0, -1.0);

        let light_intensity = color(1.0, 1.0, 1.0);
        let light_position = Tuple::set_point(0.0, 10.0, -10.0);
        let point_light = PointLight::set_point_light(light_intensity, light_position);
        let actual = lighting(m, point_light, position, eye_vector, normal_vector);

        let expected = color(1.6364, 1.6364, 1.6364);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lighting_behind_surface() {
        let m = Material::default_material();
        let position = Tuple::set_point(0.0, 0.0, 0.0);

        let eye_vector = Tuple::set_vector(0.0, 0.0, -1.0);
        let normal_vector = Tuple::set_vector(0.0, 0.0, -1.0);

        let light_intensity = color(1.0, 1.0, 1.0);
        let light_position = Tuple::set_point(0.0, 0.0, 10.0);
        let point_light = PointLight::set_point_light(light_intensity, light_position);
        let actual = lighting(m, point_light, position, eye_vector, normal_vector);

        let expected = color(0.1, 0.1, 0.1);

        assert_eq!(actual, expected);
    }
}
