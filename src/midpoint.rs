

pub mod midpoint_terrain {
    use rand::{thread_rng, Rng};
    pub fn new(exponent: u32) -> Vec<Vec<f64>> {
        let len= (usize::pow(2, exponent)+1).try_into().unwrap();
        let fill: f64 = 0.0;
        let matrix = vec![vec![fill; len]; len];
        return generate(matrix, exponent);
    }
    pub fn get_dimensions(matrix: &Vec<Vec<f64>>) -> (usize, usize) {
        return (matrix.len() - 1, matrix[0].len() - 1)
    }
    fn random() -> f64 {
        let mut rng = thread_rng();
        let rtn: f64 = rng.gen();
        return rtn;
    }
    fn random_near_zero(spread: f64) -> f64 {
        return (spread * random() * 2.0) - spread;
    }
    fn jitter(value: f64, spread: f64) -> f64 {
        return value + random_near_zero(spread);
    }
    fn init_corners(incoming_matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut matrix = incoming_matrix;
        let (x_max, y_max) = get_dimensions(&matrix);
        matrix[0][0] = random();
        matrix[0][y_max] = random();
        matrix[x_max][0] = random();
        matrix[x_max][y_max] = random();
        return matrix;
    }
    fn generate(incoming_matrix: Vec<Vec<f64>>, exponent: u32) -> Vec<Vec<f64>> {
        let mut matrix = init_corners(incoming_matrix);
        let (x_max, y_max) = get_dimensions(&matrix);
        let mut spread = 0.3;
        let mut i = 0;
        while i < exponent {
            let chunks = usize::pow(2, i);
            let chunk_width_x = x_max  / chunks;
            let chunk_width_y = y_max / chunks;

            for x_chunk in 0..chunks {
                for y_chunk in 0..chunks {
                    let left_x = chunk_width_x * x_chunk;
                    let right_x = left_x + chunk_width_x;
                    let bottom_y = chunk_width_y * y_chunk;
                    let top_y = bottom_y + chunk_width_y;
                    matrix = displace(matrix, left_x, right_x, bottom_y, top_y, spread);
                };
            };
            i += 1;
            spread *= 0.5;
        }
        return matrix;
    }
    fn displace(incoming_matrix: Vec<Vec<f64>>, left_x: usize, right_x: usize, bottom_y: usize, top_y: usize, spread: f64) -> Vec<Vec<f64>> {
        let mut matrix = incoming_matrix;
        let center_x = (left_x+right_x)/2;
        let center_y = (bottom_y+top_y)/2;

        let bottom_left = matrix[left_x][bottom_y];
        let bottom_right = matrix[right_x][bottom_y];
        let top_left = matrix[left_x][top_y];
        let top_right = matrix[right_x][top_y];

        let top = (top_left+top_right)/2.0;
        let left = (top_left+bottom_left)/2.0;
        let right = (top_right+bottom_right)/2.0;
        let bottom = (bottom_left+bottom_right)/2.0;
        let center = (top+left+right+bottom)/4.0;

        matrix[center_x][bottom_y] = jitter(bottom, spread);
        matrix[center_x][top_y] = jitter(top, spread);
        matrix[left_x][center_y] = jitter(left, spread);
        matrix[right_x][center_y] = jitter(right, spread);
        matrix[center_x][center_y] = jitter(center, spread);

        return matrix;
    }
}