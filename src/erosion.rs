pub mod erosion_mod {
    #[path="../../utils.rs"]
    pub mod utils;
    use utils::matrix_utils::{get_dimensions, get_surface_normal};
    use rand::{thread_rng, Rng};

    const PARTICLE_COUNT_MULTIPLIER: u32 = 1;
    const DENSITY: f64 = 0.01;
    const EVAP_RATE: f64 = 0.01;
    const DEPOSITION_RATE: f64 = 0.2;
    const MIN_VOL: f64 = 0.01;
    const FRICTION: f64 = 0.05;
    const TIME_SCALE: f64 = 1.2;
    #[derive(Debug)]
    pub struct Particle {
        volume: f64,
        sediment: f64,
        speed_x: f64,
        speed_y: f64,
        pos_x: usize,
        pos_y: usize
    }
    pub fn erode(incoming_matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut matrix = incoming_matrix;
        let mut rng = thread_rng();
        let mut total_removed = 0.0;


        let (x_max, y_max) = get_dimensions(&matrix);
        let part_count = x_max as u32*y_max as u32*PARTICLE_COUNT_MULTIPLIER;
        println!("Part Count: {:?}", part_count);

        for _i in 0..part_count {
            let mut part = Particle {
                volume: 1.0,
                sediment: 0.0,
                speed_x: 0.0,
                speed_y: 0.0,
                pos_x: rng.gen_range(0..x_max), 
                pos_y: rng.gen_range(0..y_max)
            };


            while part.volume > MIN_VOL {
                // println!("-----");
                // println!("Part: {:?}", part);
                let i_x = part.pos_x.clone();
                let i_y = part.pos_y.clone();
                let (nx, ny) = get_surface_normal(&matrix, i_x, i_y);

                // println!("spd x: {:?}", nx/(part.volume*DENSITY));
                // println!("spd y: {:?}", ny/(part.volume*DENSITY));

                // accelerate particle down slope
                part.speed_x += nx/(part.volume*DENSITY);
                part.speed_y += ny/(part.volume*DENSITY);
                // offset particle by it's velocity
                let new_pos_x = part.pos_x as f64 + (part.speed_x);
                let new_pos_y = part.pos_y as f64 + (part.speed_y);
                part.pos_x =  new_pos_x as usize;
                part.pos_y = new_pos_y as usize;
                // slow particle by friction
                part.speed_x *= 1.0*TIME_SCALE*FRICTION;
                part.speed_y *= 1.0*TIME_SCALE*FRICTION;

                // println!("Part: {:?}", part);

                // break if particle out of bounds
                if!(part.pos_x < x_max && part.pos_y < y_max){
                    // println!("Out of Bounds");
                    break;
                }
                // sediment calcs
                let d_z = (matrix[i_x][i_y] - matrix[part.pos_x][part.pos_y]).abs();
                let abs_speed = (part.speed_x + part.speed_y).abs();
                // println!("d_z: {:?} - {:?} - {:?}", d_z, matrix[i_x][i_y], matrix[part.pos_x][part.pos_y]);
                let mut max_sediment = part.volume*abs_speed*d_z;
                // println!("Max Sed: {:?}", max_sediment);
                if max_sediment < 0.0 {
                    max_sediment = 0.0;
                }

                let sediment_diff = (max_sediment - part.sediment)*TIME_SCALE*DEPOSITION_RATE;

                // update sediment on particle
                part.sediment += sediment_diff;
                // pull sediment from heightmap
                let to_remove = sediment_diff*part.volume;
                matrix[part.pos_x][part.pos_y] -= to_remove;
                total_removed += to_remove;
                // println!("Removed: {:?}", to_remove);

                // evaporate drop
                part.volume *= 1.0-(TIME_SCALE*EVAP_RATE);
                // println!("Part: {:?}", part);
            }
        }
        println!("Total Removed: {:?}", total_removed);
        return matrix;
    }
}