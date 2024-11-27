use crate::tuple::Tuple;
#[derive(Debug)]
pub struct Ray {
    direction: Tuple,
    origin: Tuple,
}

impl Ray {
    pub fn set_ray(direction: Tuple, origin: Tuple) -> Ray {
        Ray {
            direction: direction,
            origin: origin,
        }
    }

    pub fn calculate_position(&self, t: f64) -> Tuple {
        self.origin.clone() + (self.direction.clone() * t)
    }

    pub fn sphere_intersect(ray: &Ray, sphere: &Sphere) -> Vec<f64> {
        //return t values for intersections
        let sphere_to_ray = ray.origin.clone() - sphere.origin.clone();
        let a = Tuple::dot(ray.direction.clone(), ray.direction.clone());
        let b = 2.0 * Tuple::dot(sphere_to_ray.clone(), ray.direction.clone());
        let c = Tuple::dot(sphere_to_ray.clone(), sphere_to_ray.clone())
            - sphere.radius * sphere.radius;

        let discriminant: f64 = (b * b) - (4.0 * a * c);
        if discriminant < 0.0 {
            return Vec::new();
        }
        let t1 = (-1.0 * b - f64::sqrt(discriminant)) / (2.0 * a);
        let t2 = (-1.0 * b + f64::sqrt(discriminant)) / (2.0 * a);
        vec![t1, t2]
    }
}

#[derive(Clone)]
pub struct Sphere {
    origin: Tuple,
    radius: f64,
}

impl Sphere {
    pub fn set_sphere(origin: Tuple, radius: f64) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
        }
    }
    pub fn normal_at(&self, point: Tuple) -> Tuple {
        let vector_to_point = point - self.origin.clone();
        return vector_to_point.normalize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_struct() {
        let origin = Tuple::set_point(1.0, 2.0, 3.0);
        let direction = Tuple::set_vector(4.0, 5.0, 6.0);
        let ray = Ray::set_ray(direction, origin);

        let expected_origin = Tuple::set_point(1.0, 2.0, 3.0);
        let expected_direction = Tuple::set_vector(4.0, 5.0, 6.0);
        assert_eq!(ray.origin, expected_origin);
        assert_eq!(ray.direction, expected_direction);
    }

    #[test]
    fn test_calculate_position() {
        let origin = Tuple::set_point(2.0, 3.0, 4.0);
        let direction = Tuple::set_vector(1.0, 0.0, 0.0);
        let ray = Ray::set_ray(direction, origin);

        let ray_calc_0 = Tuple::set_point(2.0, 3.0, 4.0);
        assert_eq!(ray_calc_0, ray.calculate_position(0.0));

        let ray_calc_1 = Tuple::set_point(3.0, 3.0, 4.0);
        assert_eq!(ray_calc_1, ray.calculate_position(1.0));

        let ray_calc_neg_1 = Tuple::set_point(1.0, 3.0, 4.0);
        assert_eq!(ray_calc_neg_1, ray.calculate_position(-1.0));

        let ray_calc_25 = Tuple::set_point(4.5, 3.0, 4.0);
        assert_eq!(ray_calc_25, ray.calculate_position(2.5));
    }

    #[test]
    fn test_intersection_2_points() {
        let sphere_origin = Tuple::set_point(0.0, 0.0, 0.0);
        let test_sphere = Sphere::set_sphere(sphere_origin, 1.0);

        let ray_origin = Tuple::set_point(0.0, 0.0, -5.0);
        let ray_direction = Tuple::set_vector(0.0, 0.0, 1.0);
        let test_ray = Ray::set_ray(ray_direction, ray_origin);
        let return_value = Ray::sphere_intersect(&test_ray, &test_sphere);
        assert_eq!(return_value.len(), 2);
        assert_eq!(return_value[0], 4.0);
        assert_eq!(return_value[1], 6.0);
    }

    #[test]
    fn test_intersection_tangent() {
        // returns {4.0, 6.0 basically}
        let sphere_origin = Tuple::set_point(0.0, 0.0, 0.0);
        let test_sphere = Sphere::set_sphere(sphere_origin, 1.0);

        let ray_origin = Tuple::set_point(0.0, 1.0, -5.0);
        let ray_direction = Tuple::set_vector(0.0, 0.0, 1.0);
        let test_ray = Ray::set_ray(ray_direction, ray_origin);
        let return_value = Ray::sphere_intersect(&test_ray, &test_sphere);
        assert_eq!(return_value.len(), 2);
        assert_eq!(return_value[0], 5.0);
        assert_eq!(return_value[1], 5.0);
    }

    #[test]
    fn test_intersection_miss() {
        let sphere_origin = Tuple::set_point(0.0, 0.0, 0.0);
        let test_sphere = Sphere::set_sphere(sphere_origin, 1.0);

        let ray_origin = Tuple::set_point(0.0, 2.0, -5.0);
        let ray_direction = Tuple::set_vector(0.0, 0.0, 1.0);
        let test_ray = Ray::set_ray(ray_direction, ray_origin);
        let return_value = Ray::sphere_intersect(&test_ray, &test_sphere);
        assert_eq!(return_value.len(), 0);
    }

    #[test]
    fn test_intersection_inside_sphere() {
        let sphere_origin = Tuple::set_point(0.0, 0.0, 0.0);
        let test_sphere = Sphere::set_sphere(sphere_origin, 1.0);

        let ray_origin = Tuple::set_point(0.0, 0.0, 0.0);
        let ray_direction = Tuple::set_vector(0.0, 0.0, 1.0);
        let test_ray = Ray::set_ray(ray_direction, ray_origin);
        let return_value = Ray::sphere_intersect(&test_ray, &test_sphere);
        assert_eq!(return_value.len(), 2);
        assert_eq!(return_value[0], -1.0);
        assert_eq!(return_value[1], 1.0);
    }

    #[test]
    fn test_intersection_behind_ray() {
        let sphere_origin = Tuple::set_point(0.0, 0.0, 0.0);
        let test_sphere = Sphere::set_sphere(sphere_origin, 1.0);

        let ray_origin = Tuple::set_point(0.0, 0.0, 5.0);
        let ray_direction = Tuple::set_vector(0.0, 0.0, 1.0);
        let test_ray = Ray::set_ray(ray_direction, ray_origin);
        let return_value = Ray::sphere_intersect(&test_ray, &test_sphere);
        assert_eq!(return_value.len(), 2);
        assert_eq!(return_value[0], -6.0);
        assert_eq!(return_value[1], -4.0);
    }

    #[test]
    fn test_sphere_normal_x_axis() {
        let sphere_origin = Tuple::set_point(0.0, 0.0, 0.0);
        let test_sphere = Sphere::set_sphere(sphere_origin, 1.0);

        let test_point = Tuple::set_point(1.0, 0.0, 0.0);
        let actual_value = test_sphere.normal_at(test_point);
        let expected_value = Tuple::set_vector(1.0, 0.0, 0.0);
        assert_eq!(actual_value, expected_value);
    }

    #[test]
    fn test_sphere_normal_y_axis() {
        let sphere_origin = Tuple::set_point(0.0, 0.0, 0.0);
        let test_sphere = Sphere::set_sphere(sphere_origin, 1.0);

        let test_point = Tuple::set_point(0.0, 1.0, 0.0);
        let actual_value = test_sphere.normal_at(test_point);
        let expected_value = Tuple::set_vector(0.0, 1.0, 0.0);
        assert_eq!(actual_value, expected_value);
    }

    #[test]
    fn test_sphere_normal_z_axis() {
        let sphere_origin = Tuple::set_point(0.0, 0.0, 0.0);
        let test_sphere = Sphere::set_sphere(sphere_origin, 1.0);

        let test_point = Tuple::set_point(0.0, 0.0, 1.0);
        let actual_value = test_sphere.normal_at(test_point);
        let expected_value = Tuple::set_vector(0.0, 0.0, 1.0);
        assert_eq!(actual_value, expected_value);
    }

    #[test]
    fn test_sphere_normal_nonaxial() {
        let sphere_origin = Tuple::set_point(0.0, 0.0, 0.0);
        let test_sphere = Sphere::set_sphere(sphere_origin, 1.0);

        let test_point = Tuple::set_point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        let actual_value = test_sphere.normal_at(test_point);
        let expected_value = Tuple::set_vector(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        assert_eq!(actual_value, expected_value);
    }

    #[test]
    fn test_sphere_normal_is_normalized() {
        let sphere_origin = Tuple::set_point(0.0, 0.0, 0.0);
        let test_sphere = Sphere::set_sphere(sphere_origin, 1.0);

        let test_point = Tuple::set_point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        let actual_value = test_sphere.normal_at(test_point);
        let expected_value = actual_value.normalize();
        assert_eq!(actual_value, expected_value);
    }

    #[test]
    fn test_sphere_normal_not_origin() {
        let sphere_origin = Tuple::set_point(0.0, 1.0, 0.0);
        let test_sphere = Sphere::set_sphere(sphere_origin, 1.0);

        let test_point = Tuple::set_point(0.0, 1.70711, -0.70711);
        let actual_value = test_sphere.normal_at(test_point);
        let expected_value = Tuple::set_vector(0.0, 0.70711, -0.70711);
        assert_eq!(actual_value, expected_value);
    }
}
