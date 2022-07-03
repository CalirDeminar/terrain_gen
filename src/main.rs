mod midpoint;
mod graphing;
use midpoint::midpoint_terrain;
use graphing::graphing_mod::graph_matrix_to_gif;

fn main() {
    let matrix: Vec<Vec<f64>> = midpoint_terrain::new(6);
    println!("{:?}", midpoint_terrain::get_dimensions(&matrix));
    graph_matrix_to_gif(matrix);    
}
