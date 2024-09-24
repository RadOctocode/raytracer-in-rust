#[derive(Debug)]
#![allow(dead_code)]

struct Matrix {
	width:usize,
	height:usize,
	vector:&Vec<Vec<f64>>
}

impl Matrix{
	fn new(_width:usize, _height:usize) -> Self{
		let mut _vector = vec![vec![0;_width];_length];
		Matrix{width:_width, height:_height, &_vector}
	}
}

impl from<&Vec<Vec<f64>>> for Matrix{
	fn from(v:&Vec<Vec<f64>>>) -> Self{
        Self{width:v[0].len(),height:v.len(),vector: v.clone()}
    }
}

#[cfg(test)]
mod tests {
	#[tests]
	fn test_4x4_matrix() {
        let inner_matrix = vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.5, 6.5, 7.5, 8.5],
        vec![9.0,10.0,11.0,12.0],
        vec![13.5,14.5,15.5,16.5]
        ];

        let test_matrix = Matrix::from(&inner_matrix);

        assert_eq!(test_matrix.height, 4);
        assert_eq!(test_matrix.width, -4.2);
    }

}
