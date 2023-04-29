pub mod erosion_mod {
    use crate::utils;
    use rand::{thread_rng, Rng};
    use ndarray::Array2;
    use utils::matrix_utils::*;

    const PARTICLE_COUNT_MULTIPLIER: usize = 2;
    const DENSITY: f64 = 0.1;
    const EVAP_RATE: f64 = 0.01;
    // default dep_rate = 0.1
    const DEPOSITION_RATE: f64 = 0.4;
    const MIN_VOL: f64 = 0.01;
    const FRICTION: f64 = 0.05;
    // default time_scale = 1.2
    const TIME_SCALE: f64 = 2.0;

    #[derive(Debug)]
    pub struct Particle {
        volume: f64,
        sediment: f64,
        speed: [f64; 2],
        pos: [usize; 2]
    }
    pub fn erode(incoming_matrix: Array2<f64>, part_multipler: usize) -> Array2<f64> {
        let mut matrix = incoming_matrix;
        let mut rng = thread_rng();

        let limit = matrix.shape()[0]-1;
        println!("pre-part_count");
        let part_count = (limit*limit*part_multipler)/10;

        println!("part count: {}", part_count);
        for i in 0..part_count {
            let percentage = ((i as f64))/(part_count as f64)*100.0;
            println!("{}/{} - {}%", i, part_count, percentage as i64);
            let mut particle = Particle {
                volume: 1.0,
                sediment: 0.0,
                speed: [0.0, 0.0],
                pos: [rng.gen_range(0..limit), rng.gen_range(0..limit)]
            };
            // println!("----");
            while particle.volume > MIN_VOL {
                let initial_pos = particle.pos;
                let (nx, ny) = get_surface_normal(&matrix, particle.pos[0], particle.pos[1]);

                // particle accelerates and moves over terrain
                let mass = particle.volume * DENSITY;
                particle.speed = [
                    particle.speed[0] + nx/mass, 
                    particle.speed[1] + ny/mass
                ];
 
                let cur_x = particle.pos[0] as f64;
                let cur_y = particle.pos[1] as f64;
                particle.pos = [
                    ((cur_x + (particle.speed[0])).floor()) as usize,
                    ((cur_y + (particle.speed[1])).floor()) as usize
                ];
                particle.speed = [
                    particle.speed[0]*1.0*TIME_SCALE*FRICTION,
                    particle.speed[1]*1.0*TIME_SCALE*FRICTION,
                ];

                // println!("{:?}", particle);

                // break if out of bounds
                if particle.pos[0] > limit || particle.pos[1] > limit  {
                    break;
                }

                // pick up sediment
                let d_z = matrix[initial_pos]-matrix[particle.pos];
                let abs_speed = (particle.speed[0] + particle.speed[1]).abs();

                let mut max_sediment = particle.volume * abs_speed * d_z;
                if max_sediment < 0.0{
                    max_sediment = 0.0;
                }

                let sediment_diff = 
                    (max_sediment - particle.sediment) 
                    * TIME_SCALE 
                    * DEPOSITION_RATE;

                particle.sediment += sediment_diff * particle.volume;

                // TODO - calculate path along with the particle traveled this cycle
                //  remove height from along the entire path

                let to_remove = sediment_diff * particle.volume;
                matrix[particle.pos] -= to_remove;

                // evap volume away
                particle.volume *= 1.0-(TIME_SCALE*EVAP_RATE);
            }
        }
        return normalise(matrix);
    }
}