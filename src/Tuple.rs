#![allow(dead_code)]
use float_cmp::approx_eq;
#[derive(Debug)]
struct Tuple {
    x:f64,
    y:f64,
    z:f64,
    w:f64,
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self)->bool{
        let x_comp = approx_eq!(f64, self.x, other.x, ulps=2);
        let y_comp = approx_eq!(f64, self.y, other.y, ulps=2);
        let z_comp = approx_eq!(f64, self.z, other.z, ulps=2);
        let w_comp = approx_eq!(f64, self.w, other.w, ulps=2);

        return x_comp && y_comp && z_comp && w_comp;
    }
}

impl std::ops::Add<Tuple> for Tuple{
    type Output = Tuple;

    fn add(self, _rhs: Tuple) -> Tuple{
        tuple(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z, self.w + _rhs.w)
    }
}

impl std::ops::Sub<Tuple> for Tuple{
    type Output = Tuple;

    fn sub(self, _rhs: Tuple) -> Tuple{
        tuple(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z, self.w - _rhs.w)
    }
}

impl std::ops::Neg for Tuple{
    type Output = Tuple;

    fn neg(self) -> Tuple{
        tuple(-self.x,-self.y,-self.z,-self.w)
    }
}

impl std::ops::Mul<f64> for Tuple{
    type Output = Tuple;

    fn mul(self, _rhs: f64) -> Tuple{
        tuple(self.x * _rhs, self.y * _rhs, self.z * _rhs, self.w * _rhs)
    }
}

impl std::ops::Mul<Tuple> for f64{
    type Output = Tuple;

    fn mul(self, _rhs: Tuple) -> Tuple{
        tuple(_rhs.x * self, _rhs.y * self, _rhs.z * self, _rhs.w * self)
    }
}

impl std::ops::Div<f64> for Tuple{
    type Output = Tuple;

    fn div(self, _rhs: f64) -> Tuple{
        tuple(self.x / _rhs, self.y / _rhs, self.z / _rhs, self.w / _rhs)
    }
}

fn vector_or_point(current_tuple: Tuple) -> &'static str{
    if current_tuple.w == 1.0{
        return "point";
    }
    "vector"
}

fn vector(_x:f64, _y:f64, _z:f64) -> Tuple{
    Tuple{x:_x, y:_y, z:_z, w:0.0}
}

fn point(_x:f64, _y:f64, _z:f64) -> Tuple{
    Tuple{x:_x, y:_y, z:_z, w:1.0}
}

fn tuple(_x:f64, _y:f64, _z:f64, _w:f64) -> Tuple{
    Tuple{x:_x, y:_y, z:_z, w:_w}
}

fn magnitude(_tuple: Tuple) -> f64{
    println!("{:#?}", _tuple);
    let x_sq = _tuple.x.powf(2.0);
    let y_sq = _tuple.y.powf(2.0);
    let z_sq = _tuple.z.powf(2.0);
    let w_sq = _tuple.w.powf(2.0);
    f64::sqrt(x_sq + y_sq + z_sq + w_sq)
}

fn normalize(_tuple: Tuple) -> Tuple{
    let _x = _tuple.x;
    let _y = _tuple.y;
    let _z = _tuple.z;
    let _w = _tuple.w;
    let tuple_mag = magnitude(_tuple);
    tuple(_x/tuple_mag, _y/tuple_mag, _z/tuple_mag, _w/tuple_mag)
}

fn dot(_a:Tuple, _b:Tuple) -> f64{
    let x_prod = _a.x *_b.x;
    let y_prod = _a.y * _b.y;
    let z_prod = _a.z * _b.z;
    let w_prod = _a.w * _b.w;
    return x_prod + y_prod + z_prod + w_prod;
}

fn cross(_a:Tuple, _b:Tuple) -> Tuple{
    let x = _a.y * _b.z - _a.z * _b.y;
    let y = _a.z * _b.x - _a.x * _b.z;
    let z = _a.x * _b.y - _a.y * _b.x;
    return vector(x, y, z);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tuple_attributes_point() {
        let sample_tuple = Tuple{x:4.3, y:-4.2, z:3.1, w:1.0};
        assert_eq!(sample_tuple.x, 4.3);
        assert_eq!(sample_tuple.y, -4.2);
        assert_eq!(sample_tuple.z, 3.1);
        assert_eq!(sample_tuple.w, 1.0);
        assert_eq!(vector_or_point(sample_tuple), "point")
    }

    #[test]
    fn test_tuple_attributes_vector() {
        let sample_tuple = Tuple{x:4.3, y:-4.2, z:3.1, w:0.0};
        assert_eq!(sample_tuple.x, 4.3);
        assert_eq!(sample_tuple.y, -4.2);
        assert_eq!(sample_tuple.z, 3.1);
        assert_eq!(sample_tuple.w, 0.0);
        assert_eq!(vector_or_point(sample_tuple), "vector")
    }

    #[test]
    fn test_point_function() {
        let actual = point(4.3, -4.2, 3.1);
        let expected = Tuple{x:4.3, y:-4.2, z:3.1, w:1.0};
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_vector_function() {
        let actual = vector(4.3, -4.2, 3.1);
        let expected = Tuple{x:4.3, y:-4.2, z:3.1, w:0.0};
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_adding_tuple(){
        let a = Tuple{x:3.0, y:-2.0, z:5.0, w:1.0};
        let b = Tuple{x:-2.0, y:3.0, z:1.0, w:0.0};
        let expected = Tuple{x:1.0, y:1.0, z:6.0, w:1.0};
        let actual = a + b;
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_subtracting_points(){
        let a = point(3.0,2.0,1.0);
        let b = point(5.0,6.0,7.0);
        let expected = vector(-2.0, -4.0, -6.0);
        let actual_sum = a - b;
        assert_eq!(expected, actual_sum);
    }
    #[test]

    fn test_subtracting_point_from_vector(){
        let a = point(3.0,2.0,1.0);
        let b = vector(5.0,6.0,7.0);
        let expected = point(-2.0, -4.0, -6.0);
        let actual = a - b;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_subtracting_vectors(){
        let a = vector(3.0,2.0,1.0);
        let b = vector(5.0,6.0,7.0);
        let expected = vector(-2.0, -4.0, -6.0);
        let actual = a - b;
        assert_eq!(expected, actual);
    }

    #[test]
    fn negating_tuples(){
        let a = tuple(3.0, 2.0, 1.0, 1.0);
        let expected = tuple(-3.0, -2.0, -1.0, -1.0);
        let actual = -a;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_scalar_multiplication(){
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        let expected = tuple(3.5, -7.0, 10.5, -14.0);
        let actual = 3.5 * a;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_fraction_multiplication(){
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        let expected = tuple(0.5, -1.0, 1.5, -2.0);
        let actual = a * 0.5;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_division(){
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        let expected = tuple(0.5, -1.0, 1.5, -2.0);
        let actual = a / 2.0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_magnitude_unit_vector(){
        let a = vector(0.0, 1.0, 0.0);
        let mag_a = magnitude(a);
        assert_eq!(mag_a, 1.0);

        let b = vector(1.0, 0.0,0.0);
        let mag_b = magnitude(b);
        assert_eq!(mag_b, 1.0);

        let c = vector(0.0, 0.0, 1.0);
        let mag_c = magnitude(c);
        assert_eq!(mag_c, 1.0);
    }

    #[test]
    fn test_magnitude_vector(){
        let a = vector(1.0,2.0,3.0);
        let mag_a = magnitude(a);
        assert_eq!(mag_a, f64::sqrt(14.0));

        let b = vector(-1.0,-2.0,-3.0);
        let mag_b = magnitude(b);
        assert_eq!(mag_b, f64::sqrt(14.0));
    }

    #[test]
    fn test_normalize_vector(){
        let a = vector(4.0, 0.0, 0.0);
        let normalize_a = normalize(a);
        assert_eq!(normalize_a, vector(1.0, 0.0, 0.0));
        
        let b = vector(1.0, 2.0, 3.0);
        let normalize_b = normalize(b);
        let x = 1.0/f64::sqrt(14.0);
        let y = 2.0/f64::sqrt(14.0);
        let z = 3.0/f64::sqrt(14.0);
        assert_eq!(normalize_b, vector(x, y, z));
    }

    #[test]
    fn test_dot_product(){
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(dot(a, b), 20.0);
    }

    #[test]
    fn test_cross_product(){
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        let a_b_product = vector(-1.0, 2.0, -1.0);
        assert_eq!(cross(a,b), a_b_product);

        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        let b_a_product = vector(1.0, -2.0, 1.0);
        assert_eq!(cross(b,a), b_a_product);
    }


}