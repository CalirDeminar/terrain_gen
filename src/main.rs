
pub mod utils;
pub mod graphing;
pub mod terrain;
use terrain::terrain::*;
use utils::matrix_utils::*;

fn main() {
    gen_terrain();
    println!("----");
}
