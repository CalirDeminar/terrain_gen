pub mod matrix_utils {
    pub fn safe_access(matrix: &Vec<Vec<f64>>, x: i32, y: i32, fallback: f64) -> f64 {
        let x_safe = x >= 0 && x < (matrix.len()-1) as i32;
        let y_safe = y >= 0 && y < (matrix.len()-1) as i32;
        if x_safe && y_safe {
            return matrix[x as usize][y as usize];
        }
        return fallback;
    }
    pub fn normalise(incoming_matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut matrix = incoming_matrix;
        let mut max: f64 = 0.0;
        let mut min: f64 = f64::MAX;
        for x in 0..matrix.len() {
            for y in 0..matrix[x].len() {
                let val = matrix[x][y];
                if val > max{
                    max = val;
                }
                if val < min {
                    min = val;
                }
            }
        }
        for x in 0..matrix.len() {
            for y in 0..matrix[x].len() {
                matrix[x][y] = (matrix[x][y]-min)/(max-min);
            }
        }
        return matrix;
    }
    pub fn get_surface_normal(matrix: &Vec<Vec<f64>>, x: usize, y: usize) -> (f64, f64) {
        let dx = 0.5 * (safe_access(&matrix, x as i32 - 1, y as i32, matrix[x][y]) - safe_access(&matrix, x as i32 + 1, y as i32, matrix[x][y]));
        let dy = 0.5 * (safe_access(&matrix, x as i32, y as i32 - 1, matrix[x][y]) - safe_access(&matrix, x as i32, y as i32 + 1, matrix[x][y]));
        return (dx, dy);
    }
    pub fn get_lowest_neighbor(matrix: &Vec<Vec<f64>>, x: usize, y: usize) -> (usize, usize) {
        let mut min_x: usize = 0;
        let mut min_y: usize = 0;
        let mut min = f64::MAX;
        for x_offset in -1..2 {
            for y_offset in -1..2{
                let xn = (x as i8 - x_offset) as usize;
                let yn = (y as i8 - y_offset) as usize;
                let valid_address = xn > 0 && yn > 0;
                let not_origin = xn != x && yn != y;
                if valid_address && not_origin {
                    let val = matrix[xn][yn];
                    if val < min {
                        min = val;
                        min_x = xn;
                        min_y = yn;
                    }
                }
            }
        }
        return (min_x, min_y);
    }
    pub fn get_dimensions(matrix: &Vec<Vec<f64>>) -> (usize, usize) {
        return (matrix.len() - 1, matrix[0].len() - 1)
    }
    pub fn print(matrix: &Vec<Vec<f64>>) {
        for x in 0..matrix.len() {
            println!("{:?}", matrix[x]);
        }
    }
}