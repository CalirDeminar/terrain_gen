pub mod midpoint;
pub mod utils;
pub mod erosion;
pub mod erosion_culm;
pub mod graphing;
use midpoint::midpoint_terrain::*;
use erosion::erosion_mod;
use graphing::matrix_graphing::*;
use utils::matrix_utils::*;
use erosion_culm::erosion_culmulative;

const TERRAIN_FILENAME: &str = "./test-terrain.csv";
const ERROSION_FILENAME: &str = "./test-erosion.csv";
const ERROSION_DIFF_FILENAME: &str = "./test-erosion-diff.csv";

fn gen_terrain() {
    let matrix = new();
    write_matrix(&matrix, TERRAIN_FILENAME);
    render_matrix(&matrix, &TERRAIN_FILENAME.replace(".csv", ".png"));
}

fn erode_terrain() {
    let matrix = read_matrix(TERRAIN_FILENAME, LEN);
    let eroded = erosion_mod::erode(matrix);
    
    write_matrix(&eroded, ERROSION_FILENAME);
    render_matrix(&eroded, &ERROSION_FILENAME.replace(".csv", ".png"));
}

fn erode_culm_terrain() {
    let matrix = read_matrix(TERRAIN_FILENAME, LEN);
    let eroded = erosion_culmulative::erode(matrix.clone());
    let diff = (eroded - matrix.view()) * 100.0;
    write_matrix(&diff, "./test-erosion-culm-diff.csv");
    render_matrix(&diff, &"./test-erosion-culm-diff.png");
}

fn diff_terrain() {
    let terrain = read_matrix(TERRAIN_FILENAME, LEN);
    let eroded = read_matrix(ERROSION_FILENAME, LEN);
    let diff = (eroded - terrain.view()) * 100.0;
    write_matrix(&diff, ERROSION_DIFF_FILENAME);
    render_matrix(&diff, &ERROSION_DIFF_FILENAME.replace(".csv", ".png"));
}
fn main() {
    // gen_terrain();
    // erode_terrain();
    erode_culm_terrain();
    // diff_terrain();
    println!("----");
}
