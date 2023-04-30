pub mod erosion_culmulative {
    use ndarray::*;
    use rand::{thread_rng, Rng};
    use crate::{matrix_normal, vector_matrix_sum, apply_min_float, apply_max_matrix, utils};

    // cell size constants
    const CELL_WIDTH: f64 = 1.0;
    const CELL_AREA: f64 = CELL_WIDTH * CELL_WIDTH;
    // rain constants
    const RAIN_RATE: f64 = 0.0008 * CELL_AREA;
    const EVAP_RATE: f64 = 0.0005;
    // slope constants
    const MIN_HEIGHT_GRAD: f64 = 0.05;
    const REPOSE_SLOPE: f64 = 0.03;
    const GRAVITY: f64 = 30.0;
    const GRAD_SIGMA: f64 = 0.5;
    // sediment constants
    const SEDIMENT_CAPACITY: f64 = 50.0;
    // const DIS_RATE: f64 = 0.25;
    // const DEP_RATE: f64 = 0.001;

    const DIS_RATE: f64 = 0.0025;
    const DEP_RATE: f64 = 0.00001;

    pub fn erode(incoming_matrix: Array2<f64>, iterations: usize) -> Array2<f64> {
        let (r, _, _) =  erode_debug(incoming_matrix, iterations);
        return r;
    }
    pub fn erode_debug(incoming_matrix: Array2<f64>, iterations: usize) -> (Array2<f64>, Array2<f64>, Array2<f64>) {
        let mut terrain = incoming_matrix;
        let mut rng = thread_rng();

        let len = terrain.shape()[0];
        // let iterations = (1.4 * len as f64) as usize;
        // let iterations: usize = 20;

        let mut water = Array2::<f64>::zeros((len, len));

        let mut sediment = Array2::<f64>::zeros((len, len));

        let mut velocity = Array2::<f64>::zeros((len, len));

        for i in 0..iterations {
            // let percentage = ((i as f64))/(iterations as f64)*100.0;
            // println!("{}/{} - {}%", i, iterations, percentage as i64);

            let rain: f64 = rng.gen();
            water += rain * RAIN_RATE;
            
            let terrain_grad = matrix_normal(&terrain);
            let terrain_grad_abs = vector_matrix_sum(&terrain_grad);
            let terrain_d_z = terrain.clone() - terrain_grad_abs.clone();

            let sediment_capacity = apply_min_float(&terrain_d_z, MIN_HEIGHT_GRAD) * &velocity * &water * SEDIMENT_CAPACITY;
            // println!("{:?}", sediment_capacity);

            let mut deposited_sediment = Array2::<f64>::zeros((len, len));
            
            // calc deposited sediment
            for x in 0..len{
                for y in 0..len{
                    let sed = sediment[[x, y]];
                    let d_z = terrain_d_z[[x, y]];
                    let sed_cap = sediment_capacity[[x, y]];
                    let height = terrain[[x, y]];
                    if d_z < 0.0 {
                        // pick up sediment if downhill, more the steeper
                        deposited_sediment[[x, y]] = height.min(sed);
                    } else if sed > sed_cap {
                        // drop additional sediment as oversaturated
                        deposited_sediment[[x, y]] = DEP_RATE * (sed - sed_cap);
                    } else {
                        // slow settling of sediment
                        deposited_sediment[[x, y]] = DIS_RATE * (sed - sed_cap);
                    }
                }
            }
            // deposited_sediment = apply_max_matrix(&deposited_sediment, &(terrain_d_z.clone()*(-1.0)));

            // println!("deposited_sediment");
            // utils::matrix_utils::print(&deposited_sediment);

            sediment = sediment - deposited_sediment.clone();
            terrain = terrain + deposited_sediment.clone();

            sediment = displace(sediment, &terrain_grad_abs);
            water = displace(water, &terrain_grad_abs);
            water = water * (1.0 - EVAP_RATE);

            // TODO - terrain slippage

            velocity = (GRAVITY * CELL_WIDTH) * terrain_d_z;
        }

        return (terrain, water, sediment);
    }
    fn displace(incoming_matrix: Array2<f64>, gradient: &Array2<f64>) -> Array2<f64> {
        let mut matrix = incoming_matrix;
        let len = matrix.shape()[0];

        for x in 0..len {
            for y in 0..len {
                let grad = gradient[[x, y]];
                matrix[[x, y]] = matrix[[x, y]] + grad;
            }
        }
        return matrix;
    }
}