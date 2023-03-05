pub mod midpoint;
pub mod utils;
pub mod erosion;
pub mod erosion_culm;
pub mod graphing;
pub mod viewer;
use midpoint::midpoint_terrain::*;
use erosion::erosion_mod;
use graphing::matrix_graphing::*;
use utils::matrix_utils::*;
use erosion_culm::erosion_culmulative;
use viewer::terrain_viewer::*;

const TERRAIN_FILENAME: &str = "./test-terrain.csv";
const ERODED_FILENAME: &str = "./test-erosion-culm-diff.csv";

fn gen_terrain() {
    let matrix = new();
    write_matrix(&matrix, TERRAIN_FILENAME);
    render_matrix(&matrix, &TERRAIN_FILENAME.replace(".csv", ".png"));
}

fn erode_terrain() {
    let matrix = read_matrix(TERRAIN_FILENAME, LEN);
    let eroded = erosion_mod::erode(matrix);
    
    write_matrix(&eroded,  "./test-erosion.csv");
    render_matrix(&eroded, & "./test-erosion.png");
}

fn erode_culm_terrain() {
    let matrix = read_matrix(TERRAIN_FILENAME, LEN);
    let eroded = erosion_culmulative::erode(matrix.clone());
    let diff = (eroded - matrix.view()) * 100.0;
    write_matrix(&diff, ERODED_FILENAME);
    render_matrix(&diff, &"./test-erosion-culm-diff.png");
}

fn diff_terrain() {
    let terrain = read_matrix(TERRAIN_FILENAME, LEN);
    let eroded = read_matrix(ERODED_FILENAME, LEN);
    let diff = (eroded - terrain.view()) * 100.0;
    write_matrix(&diff, "./test-erosion-diff.csv");
    render_matrix(&diff, &"./test-erosion-diff.png");
}
fn view_terrain() {
    let terrain = read_matrix(TERRAIN_FILENAME, LEN);
    view(&terrain);
}
fn main() {
    // gen_terrain();
    // erode_terrain();
    // erode_culm_terrain();
    // diff_terrain();
    view_terrain();
    println!("----");
}
