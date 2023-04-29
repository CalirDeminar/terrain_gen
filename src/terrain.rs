pub mod midpoint;
pub mod erosion;
pub mod erosion_culm;
pub mod terrain {
    use crate::terrain::midpoint::midpoint_terrain::*;
    use crate::terrain::erosion::erosion_mod;
    use crate::terrain::erosion_culm::erosion_culmulative;
    use crate::utils::matrix_utils::*;
    use crate::graphing::matrix_graphing::*;
    const TERRAIN_FILENAME: &str = "./test-terrain.csv";
    const ERODED_FILENAME: &str = "./test-erosion-culm-diff.csv";

    const EXPONENT: u32 = 8;

    pub fn gen_terrain() {
        let matrix = new(EXPONENT);
        write_matrix(&matrix, TERRAIN_FILENAME);
        render_matrix(&matrix, &TERRAIN_FILENAME.replace(".csv", ".png"));
    }

    pub fn erode_terrain() {
        let matrix = read_matrix(TERRAIN_FILENAME, len_from_exponent(EXPONENT));
        let eroded = erosion_mod::erode(matrix);
        
        write_matrix(&eroded,  "./test-erosion.csv");
        render_matrix(&eroded, & "./test-erosion.png");
    }

    pub fn erode_culm_terrain() {
        let matrix = read_matrix(TERRAIN_FILENAME, len_from_exponent(EXPONENT));
        let eroded = erosion_culmulative::erode(matrix.clone());
        let diff = (eroded - matrix.view()) * 100.0;
        write_matrix(&diff, ERODED_FILENAME);
        render_matrix(&diff, &"./test-erosion-culm-diff.png");
    }

    pub fn diff_terrain() {
        let terrain = read_matrix(TERRAIN_FILENAME, len_from_exponent(EXPONENT));
        let eroded = read_matrix(ERODED_FILENAME, len_from_exponent(EXPONENT));
        let diff = (eroded - terrain.view()) * 100.0;
        write_matrix(&diff, "./test-erosion-diff.csv");
        render_matrix(&diff, &"./test-erosion-diff.png");
    }
}


mod tests {
    #[test]
    fn generate_test_image() {
        use crate::terrain::terrain::gen_terrain;
        gen_terrain();
    }
}