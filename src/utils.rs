pub mod matrix_utils {
    use ndarray::Array2;
    use ndarray_stats::*;
    pub fn print(matrix: Array2<f64>) {
        let len = matrix.shape()[0];
        for x in 0..len{
            println!("{:?}", matrix.row(x));
        }
    }
    pub fn normalise(incoming_matrix: Array2<f64>) -> Array2<f64> {
        let matrix = incoming_matrix;
        let len = matrix.shape()[0]-1;
        let mut max: f64 = 0.0;
        let mut min: f64 = f64::MAX;
        for x in 0..len{
            let row = matrix.row(x);
            let row_max = row.max().unwrap();
            let row_min = row.min().unwrap();
            if *row_max > max {
                max = *row_max;
            }
            if *row_min < min {
                min = *row_min;
            } 
        }
        
        return matrix.map(|e: &f64| (e-min)/(max-min));
    }
    pub fn get_surface_normal(matrix: &Array2<f64>, x: usize, y: usize) -> (f64, f64) {
        let [left, right, bottom, top] = get_neighbors(matrix, x, y);

        let dx = 0.5 * (matrix[left]-matrix[right]);
        let dy = 0.5 * (matrix[bottom]-matrix[top]);
        return (dx, dy);
    }
    fn get_neighbors(matrix: &Array2<f64>, x: usize, y: usize) -> [[usize; 2]; 4]{
        let index_limit = matrix.shape()[0]-1;

        let x_plus = limit_index((x as i32) + 1, index_limit);
        let y_plus = limit_index((y as i32) + 1, index_limit);
        let x_minus = limit_index((x as i32) - 1, index_limit);
        let y_minus = limit_index((y as i32) - 1, index_limit);

        return [[x_minus, y], [x_plus, y], [x, y_minus], [x, y_plus]];
    }
    fn limit_index(index: i32, limit: usize) -> usize {
        if index > limit as i32 {
            return limit;
        }
        if index < 0 {
            return 0;
        }
        return index as usize;
    }
}