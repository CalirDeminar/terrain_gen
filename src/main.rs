pub mod midpoint;
pub mod utils;
pub mod erosion;
pub mod graphing;
use midpoint::midpoint_terrain::*;
use erosion::erosion_mod::*;
use graphing::matrix_graphing::*;
fn main() {
    let matrix = new();
    let matrix2 = matrix.clone();

    let erosion = erode(matrix);
    render_matrix(&erosion, "./eroded.png");

    let diff = (erosion - matrix2.view()) * 100.0;
    render_matrix(&diff, "./water_flow.png");
    println!("----");
}
