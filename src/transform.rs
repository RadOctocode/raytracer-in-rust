use crate::matrix::Matrix;
use crate::tuple::Tuple;

fn translation(x:f64, y:f64, z:f64) -> Matrix{
	//return translation matrix
    let mut return_matrix = Matrix::identity(4,4);
    return_matrix.set_element(0, 3, x);
    return_matrix.set_element(1, 3, y);
    return_matrix.set_element(2, 3, z);
    return return_matrix;
}

fn scaling(x:f64, y:f64, z:f64) -> Matrix{
    let mut return_matrix = Matrix::identity(4,4);
    return_matrix.set_element(0, 0 ,x);
    return_matrix.set_element(1, 1, y);
    return_matrix.set_element(2, 2, z);
    return return_matrix;
}

// fn rotate_x(degrees:f64) -> Matrix{

// }

// fn rotate_y(degrees:f64) -> Matrix{

// }

// fn rotate_z(degrees:f64) -> Matrix{

// }

// fn shearing()-> Matrix{

// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translation(){
    	let translate_matrix = translation(5.0, -3.0, 2.0);
        let point_a = Tuple::set_point(-3.0, 4.0, 5.0);
        let point_b = Tuple::set_point(2.0, 1.0, 7.0);
        assert_eq!(translate_matrix * point_a, point_b);
    }

     #[test]
    fn test_translation_inverse(){
        let translate_matrix = translation(5.0, -3.0, 2.0);
        let inv = translate_matrix.invert();
        let point_a = Tuple::set_point(-3.0, 4.0, 5.0);
        let point_b = Tuple::set_point(-8.0, 7.0, 3.0);
        assert_eq!(inv * point_a, point_b);
    }

     #[test]
    fn test_translation_vector(){
        let translate_matrix = translation(5.0, -3.0, 2.0);
        let vector_a = Tuple::set_vector(-3.0, 4.0, 5.0);
        let vector_b = Tuple::set_vector(-3.0, 4.0, 5.0);
        assert_eq!(translate_matrix * vector_a, vector_b);
    }

    #[test]
    fn test_scaling_point(){
        let scaling_matrix = scaling(2.0, 3.0, 4.0);
        let point_a = Tuple::set_point(-4.0, 6.0, 8.0);
        let point_b = Tuple::set_point(-8.0, 18.0, 32.0);
        assert_eq!(scaling_matrix * point_a, point_b);
    }

    #[test]
    fn test_scaling_vector(){
        let scaling_matrix = scaling(2.0, 3.0, 4.0);
        let point_a = Tuple::set_vector(-4.0, 6.0, 8.0);
        let point_b = Tuple::set_vector(-8.0, 18.0, 32.0);
        assert_eq!(scaling_matrix * point_a, point_b);
    }

    #[test]
    fn test_scaling_inverse(){
        let scaling_matrix = scaling(2.0, 3.0, 4.0);
        let inv = scaling_matrix.invert();
        let point_a = Tuple::set_vector(-4.0, 6.0, 8.0);
        let point_b = Tuple::set_vector(-2.0, 2.0, 2.0);
        assert_eq!(inv * point_a, point_b);
    }
}