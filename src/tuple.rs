#![allow(dead_code)]
use float_cmp::approx_eq;
use std::ops::Index;

#[derive(Debug)]
pub struct Tuple {
    pub vector: Vec<f64>, //x[0], y[1], z[2], w[3]
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let x_comp = approx_eq!(f64, self[0], other[0], epsilon = 0.0001);
        let y_comp = approx_eq!(f64, self[1], other[1], epsilon = 0.0001);
        let z_comp = approx_eq!(f64, self[2], other[2], epsilon = 0.0001);
        let w_comp = approx_eq!(f64, self[3], other[3], epsilon = 0.0001);

        return x_comp && y_comp && z_comp && w_comp;
    }
}

impl std::ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Tuple {
        Self::set_tuple(vec![
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
            self[3] + rhs[3],
        ])
    }
}

impl std::ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Tuple {
        Self::set_tuple(vec![
            self[0] - rhs[0],
            self[1] - rhs[1],
            self[2] - rhs[2],
            self[3] - rhs[3],
        ])
    }
}

impl std::ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Self::set_tuple(vec![-self[0], -self[1], -self[2], -self[3]])
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Tuple {
        Self::set_tuple(vec![
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
            self[3] * rhs,
        ])
    }
}

impl std::ops::Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Tuple {
        Tuple::set_tuple(vec![
            rhs[0] * self,
            rhs[1] * self,
            rhs[2] * self,
            rhs[3] * self,
        ])
    }
}

impl std::ops::Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Tuple {
        Self::set_tuple(vec![
            self[0] / rhs,
            self[1] / rhs,
            self[2] / rhs,
            self[3] / rhs,
        ])
    }
}

impl Clone for Tuple {
    fn clone(&self) -> Tuple {
        Tuple {
            vector: self.vector.clone(),
        }
    }
}

impl Index<usize> for Tuple {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.vector[index]
    }
}

impl Tuple {
    pub fn is_vector(&self) -> bool {
        self.vector[3] == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.vector[3] == 1.0
    }

    pub fn x(&self) -> f64 {
        self.vector[0]
    }

    pub fn y(&self) -> f64 {
        self.vector[1]
    }

    pub fn z(&self) -> f64 {
        self.vector[2]
    }

    pub fn w(&self) -> f64 {
        self.vector[3]
    }

    pub fn set_vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            vector: vec![x, y, z, 0.0],
        }
    }

    pub fn set_tuple(vector: Vec<f64>) -> Tuple {
        Tuple { vector }
    }

    pub fn set_point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            vector: vec![x, y, z, 1.0],
        }
    }

    pub fn magnitude(&self) -> f64 {
        let x_sq = self[0].powf(2.0);
        let y_sq = self[1].powf(2.0);
        let z_sq = self[2].powf(2.0);
        let w_sq = self[3].powf(2.0);
        f64::sqrt(x_sq + y_sq + z_sq + w_sq)
    }
    pub fn normalize(&self) -> Tuple {
        let tuple_mag = Self::magnitude(self);
        Tuple::set_tuple(vec![
            self[0] / tuple_mag,
            self[1] / tuple_mag,
            self[2] / tuple_mag,
            self[3] / tuple_mag,
        ])
    }
    pub fn dot(a: Tuple, b: Tuple) -> f64 {
        let x_prod = a[0] * b[0];
        let y_prod = a[1] * b[1];
        let z_prod = a[2] * b[2];
        let w_prod = a[3] * b[3];
        x_prod + y_prod + z_prod + w_prod
    }

    pub fn cross(a: Tuple, b: Tuple) -> Tuple {
        let x = a[1] * b[2] - a[2] * b[1];
        let y = a[2] * b[0] - a[0] * b[2];
        let z = a[0] * b[1] - a[1] * b[0];
        Self::set_vector(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tuple_attributes_point() {
        let sample_tuple = Tuple::set_tuple(vec![4.3, -4.2, 3.1, 1.0]);
        assert_eq!(sample_tuple.x(), 4.3);
        assert_eq!(sample_tuple.y(), -4.2);
        assert_eq!(sample_tuple.z(), 3.1);
        assert_eq!(sample_tuple.w(), 1.0);
        assert_eq!(sample_tuple.is_point(), true);
        assert_eq!(sample_tuple.is_vector(), false);
    }

    #[test]
    fn test_tuple_attributes_vector() {
        let sample_tuple = Tuple::set_tuple(vec![4.3, -4.2, 3.1, 0.0]);
        assert_eq!(sample_tuple.x(), 4.3);
        assert_eq!(sample_tuple.y(), -4.2);
        assert_eq!(sample_tuple.z(), 3.1);
        assert_eq!(sample_tuple.w(), 0.0);
        assert_eq!(sample_tuple.is_vector(), true);
        assert_eq!(sample_tuple.is_point(), false);
    }

    #[test]
    fn test_point_function() {
        let actual = Tuple::set_point(4.3, -4.2, 3.1);
        let expected = Tuple::set_tuple(vec![4.3, -4.2, 3.1, 1.0]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_vector_function() {
        let actual = Tuple::set_vector(4.3, -4.2, 3.1);
        let expected = Tuple::set_tuple(vec![4.3, -4.2, 3.1, 0.0]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_adding_tuple() {
        let a = Tuple::set_tuple(vec![3.0, -2.0, 5.0, 1.0]);
        let b = Tuple::set_tuple(vec![-2.0, 3.0, 1.0, 0.0]);
        let expected = Tuple::set_tuple(vec![1.0, 1.0, 6.0, 1.0]);
        let actual = a + b;
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_subtracting_points() {
        let a = Tuple::set_point(3.0, 2.0, 1.0);
        let b = Tuple::set_point(5.0, 6.0, 7.0);
        let expected = Tuple::set_vector(-2.0, -4.0, -6.0);
        let actual_sum = a - b;
        assert_eq!(expected, actual_sum);
    }

    #[test]
    fn test_subtracting_point_from_vector() {
        let a = Tuple::set_point(3.0, 2.0, 1.0);
        let b = Tuple::set_vector(5.0, 6.0, 7.0);
        let expected = Tuple::set_point(-2.0, -4.0, -6.0);
        let actual = a - b;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_subtracting_vectors() {
        let a = Tuple::set_vector(3.0, 2.0, 1.0);
        let b = Tuple::set_vector(5.0, 6.0, 7.0);
        let expected = Tuple::set_vector(-2.0, -4.0, -6.0);
        let actual = a - b;
        assert_eq!(expected, actual);
    }

    #[test]
    fn negating_tuples() {
        let a = Tuple::set_tuple(vec![3.0, 2.0, 1.0, 1.0]);
        let expected = Tuple::set_tuple(vec![-3.0, -2.0, -1.0, -1.0]);
        let actual = -a;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_scalar_multiplication() {
        let a = Tuple::set_tuple(vec![1.0, -2.0, 3.0, -4.0]);
        let expected = Tuple::set_tuple(vec![3.5, -7.0, 10.5, -14.0]);
        let actual = 3.5 * a;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_fraction_multiplication() {
        let a = Tuple::set_tuple(vec![1.0, -2.0, 3.0, -4.0]);
        let expected = Tuple::set_tuple(vec![0.5, -1.0, 1.5, -2.0]);
        let actual = a * 0.5;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_division() {
        let a = Tuple::set_tuple(vec![1.0, -2.0, 3.0, -4.0]);
        let expected = Tuple::set_tuple(vec![0.5, -1.0, 1.5, -2.0]);
        let actual = a / 2.0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_magnitude_unit_vector() {
        let a = Tuple::set_vector(0.0, 1.0, 0.0);
        let mag_a = a.magnitude();
        assert_eq!(mag_a, 1.0);

        let b = Tuple::set_vector(1.0, 0.0, 0.0);
        let mag_b = b.magnitude();
        assert_eq!(mag_b, 1.0);

        let c = Tuple::set_vector(0.0, 0.0, 1.0);
        let mag_c = c.magnitude();
        assert_eq!(mag_c, 1.0);
    }

    #[test]
    fn test_magnitude_vector() {
        let a = Tuple::set_vector(1.0, 2.0, 3.0);
        let mag_a = a.magnitude();
        assert_eq!(mag_a, f64::sqrt(14.0));

        let b = Tuple::set_vector(-1.0, -2.0, -3.0);
        let mag_b = b.magnitude();
        assert_eq!(mag_b, f64::sqrt(14.0));
    }

    #[test]
    fn test_normalize_vector() {
        let a = Tuple::set_vector(4.0, 0.0, 0.0);
        let normalize_a = a.normalize();
        assert_eq!(normalize_a, Tuple::set_vector(1.0, 0.0, 0.0));

        let b = Tuple::set_vector(1.0, 2.0, 3.0);
        let normalize_b = b.normalize();
        let x = 1.0 / f64::sqrt(14.0);
        let y = 2.0 / f64::sqrt(14.0);
        let z = 3.0 / f64::sqrt(14.0);
        assert_eq!(normalize_b, Tuple::set_vector(x, y, z));
    }

    #[test]
    fn test_dot_product() {
        let a = Tuple::set_vector(1.0, 2.0, 3.0);
        let b = Tuple::set_vector(2.0, 3.0, 4.0);
        assert_eq!(Tuple::dot(a, b), 20.0);
    }

    #[test]
    fn test_cross_product() {
        let a = Tuple::set_vector(1.0, 2.0, 3.0);
        let b = Tuple::set_vector(2.0, 3.0, 4.0);
        let a_b_product = Tuple::set_vector(-1.0, 2.0, -1.0);
        assert_eq!(Tuple::cross(a, b), a_b_product);

        let a = Tuple::set_vector(1.0, 2.0, 3.0);
        let b = Tuple::set_vector(2.0, 3.0, 4.0);
        let b_a_product = Tuple::set_vector(1.0, -2.0, 1.0);
        assert_eq!(Tuple::cross(b, a), b_a_product);
    }
}
