pub mod midpoint_terrain {
    #[path="../../utils.rs"]
    pub mod utils;
    use ndarray::*;
    use rand::{thread_rng, Rng};
    use utils::matrix_utils::*;
    const EXPONENT: u32 = 10;
    pub const LEN: usize = usize::pow(2, EXPONENT)+1;
    pub fn new() -> Array2<f64> {
        let matrix = ndarray::Array2::<f64>::zeros((LEN, LEN));
        return generate(init_corners(matrix));
    }
    fn init_corners(incoming_matrix: Array2<f64>) -> Array2<f64> {
        let mut matrix = incoming_matrix;
        let len = matrix.shape()[0]-1;
        matrix[[0, 0]] = random();
        matrix[[0, len]] = random();
        matrix[[len, 0]] = random();
        matrix[[len, len]] = random();
        return matrix;
    }
    fn generate(incoming_matrix: Array2<f64>) -> Array2<f64> {
        let mut matrix = incoming_matrix;
        let mut spread = 0.3;
        let len = matrix.shape()[0];
        for i in 0..EXPONENT{
            let chunk_count = usize::pow(2, i);
            let chunk_width = (len - 1) / chunk_count;

            for x in 0..chunk_count {
                for y in 0..chunk_count {
                    let left_x_bound = x * chunk_width;
                    let right_x_bound = left_x_bound + chunk_width;
                    let bottom_y_bound = y * chunk_width;
                    let top_y_bound = bottom_y_bound + chunk_width;
                    matrix = displace_sector(matrix, ((left_x_bound, right_x_bound), (bottom_y_bound, top_y_bound)), spread);
                }
            }

            spread *= 0.5;
        }
        // TODO - normalise
        return normalise(matrix);
    }
    fn displace_sector(incoming_matrix: Array2<f64>, edges: ((usize, usize), (usize, usize)), spread: f64 ) -> Array2<f64> {
        let mut matrix = incoming_matrix;
        let ((left_x, right_x), (bottom_y, top_y)) = edges;
        
        let center_x = (left_x+right_x)/2;
        let center_y = (bottom_y+top_y)/2;

        let bottom_left = matrix[[left_x, bottom_y]];
        let bottom_right = matrix[[right_x, bottom_y]];
        let top_left = matrix[[left_x, top_y]];
        let top_right = matrix[[right_x, top_y]];

        let top = (top_left+top_right)/2.0;
        let left = (top_left+bottom_left)/2.0;
        let right = (top_right+bottom_right)/2.0;
        let bottom = (bottom_left+bottom_right)/2.0;
        let center = (top+left+right+bottom)/4.0;

        matrix[[center_x, bottom_y]] = jitter(bottom, spread);
        matrix[[center_x, top_y]] = jitter(top, spread);
        matrix[[left_x, center_y]] = jitter(left, spread);
        matrix[[right_x, center_y]] = jitter(right, spread);
        matrix[[center_x, center_y]] = jitter(center, spread);
        
        return matrix
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
}