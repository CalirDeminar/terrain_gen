pub mod matrix_utils {
    use ndarray::Array2;
    use ndarray_csv::{Array2Writer, Array2Reader};
    use ndarray_stats::*;
    use csv::{ReaderBuilder, WriterBuilder};
    pub fn print(matrix: &Array2<f64>) {
        let len = matrix.shape()[0];
        for x in 0..len{
            println!("{:?}", matrix.row(x).map(|c| format!("{:.*}", 5, c).parse::<f32>().unwrap()));
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
    pub fn matrix_normal(matrix: &Array2<f64>) -> Array2<(f64, f64)> {
        let shape = matrix.shape();
        let mut output = Array2::<(f64, f64)>::default((shape[0], shape[1]));
        for x in 0..shape[0] {
            for y in 0..shape[1] {
                output[[x, y]] = get_surface_normal(matrix, x, y);
            }
        }
        return output;
    }
    pub fn vector_matrix_sum(matrix: &Array2<(f64, f64)>) -> Array2<f64> {
        let shape = matrix.shape();
        let mut output = Array2::<f64>::zeros((shape[0], shape[1]));
        for x in 0..shape[0] {
            for y in 0..shape[1] {
                let val = matrix[[x, y]];
                output[[x, y]] = val.0 + val.1;
            }
        }
        return output;
}
    pub fn write_matrix(matrix: &Array2<f64>, filename: &str) {
        let file = std::fs::File::create(filename).unwrap();
        let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);
        writer.serialize_array2(&matrix).unwrap();
        writer.flush().unwrap();
    }
    pub fn read_matrix(filename: &str, len: usize) -> Array2<f64> {
        let file = std::fs::File::open(filename).unwrap();
        let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
        let output: Array2<f64> = reader.deserialize_array2((len, len)).unwrap();
        return output;
    }
    pub fn apply_min_float(incoming_matrix: &Array2::<f64>, min: f64) -> Array2::<f64> {
        let len = incoming_matrix.shape()[0];
        let mut matrix = Array2::<f64>::zeros((len, len));
        for x in 0..matrix.shape()[0] {
            for y in 0..matrix.shape()[1] {
                matrix[[x, y]] = matrix[[x, y]].max(min);
            }
        }
        return matrix;
    }
    pub fn apply_max_float(incoming_matrix: &Array2::<f64>, max: f64) -> Array2::<f64> {
        let len = incoming_matrix.shape()[0];
        let mut matrix = Array2::<f64>::zeros((len, len));
        for x in 0..matrix.shape()[0] {
            for y in 0..matrix.shape()[1] {
                matrix[[x, y]] = matrix[[x, y]].max(max);
            }
        }
        return matrix;
    }
    pub fn apply_max_matrix(incoming_matrix: &Array2::<f64>, max: &Array2::<f64>) -> Array2::<f64> {
        let len = incoming_matrix.shape()[0];
        let mut matrix = Array2::<f64>::zeros((len, len));
        for x in 0..matrix.shape()[0] {
            for y in 0..matrix.shape()[1] {
                matrix[[x, y]] = incoming_matrix[[x, y]].max(max[[x, y]]);
            }
        }
        return matrix;
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