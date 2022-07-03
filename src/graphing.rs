pub mod matrix_graphing {
    use ndarray::Array2;
    use image::*;
    pub fn render_matrix(f_matrix: &Array2<f64>, filename: &str) {
        let u_matrix = matrix_to_u8(f_matrix);
        let len = u_matrix.shape()[0]-1;
        let mut image = <image::ImageBuffer::<image::Rgb<u8>, _>>::new(len as u32, len as u32);
        for x in 0..len{
            for y in 0..len{
                let val = u_matrix[[x, y]];
                 *image.get_pixel_mut(x as u32, y as u32) = Rgb([val, val, val]);
            }
        }
        image.save(filename).unwrap();
    }
    fn matrix_to_u8(matrix: &Array2<f64>) -> Array2<u8> {
        let len = matrix.shape()[0];
        let mut new = ndarray::Array2::<u8>::zeros((len, len));
        for x in 0..len-1{
            for y in 0..len-1{
                new[[x, y]] = (matrix[[x, y]]*255.0) as u8;
            }
        }
        return new;
    }
}