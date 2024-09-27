use float_cmp::approx_eq;

struct Matrix {
    width: usize,
    height: usize,
    vector: Vec<Vec<f64>>,
}

impl Matrix {
    fn new(_width: usize, _height: usize) -> Self {
        let mut _vector = vec![vec![0.0; _width]; _height];
        Matrix {
            width: _width,
            height: _height,
            vector: _vector,
        }
    }
    fn set(v: &Vec<Vec<f64>>) -> Self {
        Self {
            width: v[0].len(),
            height: v.len(),
            vector: v.clone(),
        }
    }
    fn get(&self, y: usize, x: usize) -> f64 {
        self.vector[y][x]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.height != other.height || self.width != other.width {
            return false;
        }

        for (y_index, y_vect) in self.vector.iter().enumerate() {
            for (x_index, x_el) in y_vect.iter().enumerate() {
                if other.get(y_index, x_index) != *x_el {
                    return false;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4x4_matrix() {
        let inner_matrix = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ];

        let test_matrix = Matrix::set(&inner_matrix);

        assert_eq!(test_matrix.height, 4);
        assert_eq!(test_matrix.width, 4);
        assert_eq!(test_matrix.get(0, 0), 1.0);
        assert_eq!(test_matrix.get(0, 3), 4.0);
        assert_eq!(test_matrix.get(1, 0), 5.5);
        assert_eq!(test_matrix.get(1, 2), 7.5);
        assert_eq!(test_matrix.get(2, 2), 11.0);
        assert_eq!(test_matrix.get(3, 0), 13.5);
        assert_eq!(test_matrix.get(3, 2), 15.5);
    }

    #[test]
    fn test_2x2_matrix() {
        let inner_matrix = vec![vec![-3.0, 5.0], vec![1.0, -2.0]];
        let test_matrix = Matrix::set(&inner_matrix);
        assert_eq!(test_matrix.height, 2);
        assert_eq!(test_matrix.width, 2);
        assert_eq!(test_matrix.get(0, 0), -3.0);
        assert_eq!(test_matrix.get(0, 1), 5.0);
        assert_eq!(test_matrix.get(1, 0), 1.0);
        assert_eq!(test_matrix.get(1, 1), -2.0);
    }

    #[test]
    fn test_eq_true() {
        let matrix_a_vec = vec![vec![-3.0, 5.0], vec![1.0, -2.0]];
        let matrix_b_vec = vec![vec![-3.0, 5.0], vec![1.0, -2.0]];
        let a = Matrix::set(&matrix_a_vec);
        let b = Matrix::set(&matrix_b_vec);
        assert_eq!(a == b, true);
    }

    #[test]
    fn test_eq_false_elements() {
        let matrix_a_vec = vec![vec![-3.0, 6.0], vec![1.0, -2.0]];
        let matrix_b_vec = vec![vec![-3.0, 5.0], vec![1.0, -2.0]];
        let a = Matrix::set(&matrix_a_vec);
        let b = Matrix::set(&matrix_b_vec);
        assert_eq!(a == b, false);
    }

    #[test]
    fn test_ineq_elements() {
        let matrix_a_vec = vec![vec![-3.0, 6.0], vec![1.0, -2.0]];
        let matrix_b_vec = vec![vec![-3.0, 5.0], vec![1.0, -2.0]];
        let a = Matrix::set(&matrix_a_vec);
        let b = Matrix::set(&matrix_b_vec);
        assert_eq!(a != b, true);
    }

    #[test]
    fn test_eq_false_dimensions() {
        let matrix_a_vec = vec![vec![-3.0, 5.0, 1.0], vec![1.0, -2.0, 1.0]];
        let matrix_b_vec = vec![vec![-3.0, 5.0], vec![1.0, -2.0]];
        let a = Matrix::set(&matrix_a_vec);
        let b = Matrix::set(&matrix_b_vec);
        assert_eq!(a == b, false);
    }
}
