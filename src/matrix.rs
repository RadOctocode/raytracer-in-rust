use crate::tuple::Tuple;
use float_cmp::approx_eq;

#[derive(Debug)]
pub struct Matrix {
    width: usize,
    height: usize,
    vector: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn zero(_width: usize, _height: usize) -> Self {
        let _vector = vec![vec![0.0; _width]; _height];
        Matrix {
            width: _width,
            height: _height,
            vector: _vector,
        }
    }

    pub fn set(v: &Vec<Vec<f64>>) -> Self {
        Self {
            width: v[0].len(),
            height: v.len(),
            vector: v.clone(),
        }
    }

    pub fn identity(width: usize, height: usize) -> Self {
        let mut vector = vec![vec![0.0; width]; height];
        for i in 0..height {
            for j in 0..width {
                if i == j {
                    vector[i][j] = 1.0;
                }
            }
        }
        Matrix {
            width,
            height,
            vector,
        }
    }

    pub fn get(&self, y: usize, x: usize) -> f64 {
        self.vector[y][x]
    }

    pub fn set_element(&mut self, y: usize, x: usize, value: f64) {
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

    pub fn transpose(&self) -> Matrix {
        let mut vector = Vec::new();
        //make each row a coloum
        for i in 0..self.width {
            let current_coloum = self.get_col(i);
            vector.push(current_coloum);
        }
        Matrix {
            width: self.height,
            height: self.width,
            vector: vector,
        }
    }

    pub fn determinant(&self) -> f64 {
        if self.width == 2 && self.height == 2 {
            return (self.vector[0][0] * self.vector[1][1])
                - (self.vector[0][1] * self.vector[1][0]);
        }
        let mut determinant = 0.0;
        let top_row = self.get_row(0);
        for (row_index, row_element) in top_row.iter().enumerate() {
            determinant = determinant + (row_element * self.cofactor(0, row_index));
        }
        return determinant;
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut vector = Vec::new();
        for (row_index, row_element) in self.vector.iter().enumerate() {
            if row_index != row {
                let mut new_row = Vec::new();
                for (element_index, element) in row_element.iter().enumerate() {
                    if element_index != col {
                        new_row.push(*element);
                    }
                }
                vector.push(new_row);
            }
        }
        return Matrix::set(&vector);
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let current_sub_matrix = self.submatrix(row, col);
        return current_sub_matrix.determinant();
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let mut current_minor = self.minor(row, col);
        if (row + col) % 2 == 1 {
            current_minor = current_minor * -1.0;
        }
        return current_minor;
    }

    pub fn is_invertable(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn invert(&self) -> Matrix{
        if !self.is_invertable(){
            panic!("this matrix is not invertable");
        }
        let current_determinant = self.determinant();
        let mut return_matrix = Matrix::zero(self.width, self.height);
        for y_index in 0..self.height{
            for x_index in 0..self.width{
                // notice the (x_index, y_index) instead of (y_index, x_index)
                let current_cofactor = self.cofactor(x_index, y_index);
                return_matrix.vector[y_index][x_index] = current_cofactor/current_determinant;
            }
        }
        return return_matrix;
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.height != other.height || self.width != other.width {
            return false;
        }

        for (y_index, y_vect) in self.vector.iter().enumerate() {
            for (x_index, x_el) in y_vect.iter().enumerate() {
                let lhs = other.get(y_index, x_index);
                let rhs = *x_el;
                let float_eq = approx_eq!(f64, lhs, rhs, epsilon = 0.001);
                if !float_eq {
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
    fn mul(self, rhs: Matrix) -> Matrix {
        if self.width != rhs.height {
            panic!("left element width doesn't equal right element")
        }
        let mut output_matrix = Matrix::zero(self.height, rhs.width);
        for y_index in 0..self.height {
            for x_index in 0..rhs.width {
                let a_row = self.get_row(y_index);
                let b_col = rhs.get_col(x_index);
                let total_element = calculate_element_sum(&a_row, &b_col);
                output_matrix.set_element(y_index, x_index, total_element);
            }
        }
        return output_matrix;
    }
}

impl std::ops::Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Tuple {
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

    #[test]
    fn test_get_row() {
        let a_vec = vec![vec![-3.0, 5.0, 1.0], vec![1.0, 2.0, 1.0]];
        let a = Matrix::set(&a_vec);

        let expected_vec = vec![-3.0, 5.0, 1.0];
        let actual_vec = a.get_row(0);
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn test_get_col() {
        let a_vec = vec![vec![3.0, 5.0], vec![2.0, 1.0], vec![1.0, -2.0]];
        let a = Matrix::set(&a_vec);

        let expected_vec = vec![3.0, 2.0, 1.0];
        let actual_vec = a.get_col(0);
        assert_eq!(actual_vec, expected_vec);
    }

    #[test]
    fn test_matrix_multiplication() {
        let a_vec = vec![vec![-3.0, 5.0, 1.0], vec![1.0, 2.0, 1.0]];
        let b_vec = vec![vec![3.0, 5.0], vec![2.0, 1.0], vec![1.0, -2.0]];
        let c_vec = vec![vec![2.0, -12.0], vec![8.0, 5.0]];
        let a = Matrix::set(&a_vec);
        let b = Matrix::set(&b_vec);
        let c = Matrix::set(&c_vec);
        let actual_sum = a * b;
        assert_eq!(actual_sum, c);
    }

    #[test]
    fn test_matrix_multiplication_tuple() {
        let a_vec = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];
        let b_vec = vec![1.0, 2.0, 3.0, 1.0];
        let c_vec = vec![18.0, 24.0, 33.0, 1.0];
        let a = Matrix::set(&a_vec);
        let b = Tuple::set_tuple(b_vec);
        let c = Tuple::set_tuple(c_vec);
        let actual_sum = a * b;
        assert_eq!(actual_sum, c);
    }

    #[test]
    fn test_matrix_identity() {
        let a_vec = vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];
        let expected = Matrix::set(&a_vec);

        let actual = Matrix::identity(4, 4);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_matrix_transpose_4_x_4() {
        let a_vec = vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ];
        let expected = vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ];

        let a = Matrix::set(&a_vec);
        let expected = Matrix::set(&expected);
        assert_eq!(a.transpose(), expected);
    }

    #[test]
    fn test_matrix_transpose_3_x_2() {
        let a_vec = vec![vec![0.0, 9.0], vec![9.0, 8.0], vec![1.0, 8.0]];
        let expected = vec![vec![0.0, 9.0, 1.0], vec![9.0, 8.0, 8.0]];

        let a = Matrix::set(&a_vec);
        let expected = Matrix::set(&expected);
        assert_eq!(a.transpose(), expected);
    }

    #[test]
    fn test_matrix_determinant_2_x_2() {
        let a_vec = vec![vec![1.0, 5.0], vec![-3.0, 2.0]];
        let a = Matrix::set(&a_vec);
        assert_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn test_calculate_minor_3_x_3() {
        let a_vec = vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ];
        let a = Matrix::set(&a_vec);
        let b = a.submatrix(1, 0);
        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn test_calculate_cofactor_3_x_3() {
        let a_vec = vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ];
        let a = Matrix::set(&a_vec);
        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_calculate_determinant_3_x_3() {
        let a_vec = vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ];
        let a = Matrix::set(&a_vec);
        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn test_calculate_determinant_4_x_4() {
        let a_vec = vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ];
        let a = Matrix::set(&a_vec);
        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn test_calculate_is_invertable() {
        let a_vec = vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ];
        let b_vec = vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ];
        let a = Matrix::set(&a_vec);
        let b = Matrix::set(&b_vec);
        assert_eq!(a.determinant(), -2120.0);
        assert_eq!(a.is_invertable(), true);

        assert_eq!(b.determinant(), 0.0);
        assert_eq!(b.is_invertable(), false);
    }

    #[test]
    fn test_is_invertable_4_x_4() {
        let a_vec = vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ];
        let b_vec = vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ];
        let a = Matrix::set(&a_vec);
        let b = Matrix::set(&b_vec);
        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.is_invertable(), true);

        assert_eq!(b.determinant(), 0.0);
        assert_eq!(b.is_invertable(), false);
    }

    #[test]
    fn test_invert_4_x_4() {
        let a_vec = vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ];
        let b_vec = vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639],
        ];
        let a = Matrix::set(&a_vec);
        let b = Matrix::set(&b_vec);
        assert_eq!(a.invert(), b);
    }
}
