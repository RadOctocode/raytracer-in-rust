pub mod Tuple;

#[derive(Debug)]
struct Matrix {
    width: usize,
    height: usize,
    vector: Vec<Vec<f64>>,
}

impl Matrix {
    fn zero(_width: usize, _height: usize) -> Self {
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
    fn set_element(&mut self, y: usize, x: usize, value: f64) {
        self.vector[y][x] = value;
    }

    fn get_row(&self, index: usize) -> Vec<f64> {
        self.vector[index].clone()
    }

    fn get_col(&self, index: usize) -> Vec<f64> {
        let mut return_value = Vec::new();
        for list in &self.vector {
            return_value.push(list[index]);
        }
        return return_value;
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

fn calculate_element_sum(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    let mut sum = 0.0;

    for (index, element) in a.iter().enumerate() {
        sum += element * b[index];
    }

    return sum;
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, _rhs: Matrix) -> Matrix {
        let mut output_matrix = Matrix::zero(self.height, _rhs.width);
        for y_index in 0..self.height {
            for x_index in 0.._rhs.width {
                let a_row = self.get_row(y_index);
                let b_col = self.get_col(x_index);
                let total_element = calculate_element_sum(&a_row, &b_col);
                output_matrix.set_element(y_index, x_index, total_element);
            }
        }
        return output_matrix;
    }
}

impl std::ops::Mul<Tuple> for Matrix {
    type Output = Tuple::Tuple;
    fn mul(self, rhs: Tuple::Tuple) -> Tuple::Tuple {
        let mut output_tuple_vector = vec![0.0, 0.0, 0.0, 0.0];
        for y_index in 0..self.height {
            let matrix_row = self.get_row(y_index);
            let total_element = calculate_element_sum(&matrix_row, &rhs.vector);
            output_tuple_vector[y_index] = total_element;
        }
        return Tuple::set_tuple(output_tuple_vector);
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

    #[test]
    fn test_calculate_element_sum() {
        let a = vec![-3.0, 5.0, 1.0];
        let b = vec![3.0, 2.0, 1.0];
        let expected_sum = 2.0;
        let actual_sum = calculate_element_sum(&a, &b);
        assert_eq!(actual_sum, expected_sum);
    }

    // #[test]
    // fn test_matrix_multiplication() {
    //     let a_vec = vec![vec![-3.0, 5.0, 1.0], vec![1.0, 2.0, 1.0]];
    //     let b_vec = vec![vec![3.0, 5.0], vec![2.0, 1.0], vec![1.0, -2.0]];
    //     let c_vec = vec![vec![2.0, -12.0], vec![8.0, 5.0]];
    //     let a = Matrix::set(&a_vec);
    //     let b = Matrix::set(&b_vec);
    //     let c = Matrix::set(&c_vec);
    //     let actual_sum = a * b;
    //     assert_eq!(actual_sum, c);
    // }
}
