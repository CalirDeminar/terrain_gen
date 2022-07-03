pub mod graphing_mod {
    use plotters::prelude::*;
    use image::*;
    const OUT_FILE_NAME_SVG: &'static str = "./export.svg";
    pub fn graph_matrix_to_svg(matrix: Vec<Vec<f64>>) {
        let area = SVGBackend::new(OUT_FILE_NAME_SVG, (1024, 760)).into_drawing_area();
            area.fill(&WHITE).unwrap();

        let x_axis = 0..(matrix.len());
        let y_axis = 0..(matrix[0].len());
        let z_axis = 0.0 as f64..1.0 as f64;

        let mut chart = ChartBuilder::on(&area)
            .caption(format!("Terrain"), ("sans", 20 as u32))
            .build_cartesian_3d(x_axis.clone(), z_axis.clone(), y_axis.clone()).unwrap();

        chart.with_projection(|mut pb| {
            pb.yaw = 0.5;
            pb.scale = 0.9;
            pb.pitch = 0.75;
            pb.into_matrix()
        });

        chart.draw_series(SurfaceSeries::xoz(
            (0..matrix.len()).map(|f| f),
            (0..matrix.len()).map(|f| f),
            |x, y| matrix[x][y]
        )
        .style(BLUE.mix(0.2).filled()))
        .unwrap()
        .label("Surface")
        .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));

        chart.configure_series_labels().border_style(&BLACK).draw().unwrap();

        area.present().expect("Unable to write");
    }
    pub fn graph_matrix_to_gif(matrix: Vec<Vec<f64>>, filename: &str) {

        let root = BitMapBackend::gif(&filename, (1920, 1080), 100).unwrap().into_drawing_area();

        for pitch in (0..157).step_by(5){
            root.fill(&WHITE).unwrap();

            
            let x_axis = 0..(matrix.len());
            let y_axis = 0..(matrix[0].len());
            let z_axis = 0.0 as f64..1.0 as f64;

            let mut chart = ChartBuilder::on(&root)
                .caption("Terrain", ("sans-serif",  2))
                .build_cartesian_3d(x_axis.clone(), z_axis.clone(), y_axis.clone())
                .unwrap();

            chart.with_projection(|mut p| {
                p.pitch = 1.57 - (1.57 - pitch as f64 / 50.0).abs();
                p.scale = 0.7;
                p.into_matrix() // build the projection matrix
            });  

            chart
                .configure_axes()
                .light_grid_style(BLACK.mix(0.15))
                .draw()
                .unwrap();

            chart.draw_series(
                SurfaceSeries::xoz(
                (0..matrix.len()).map(|f| f),
                (0..matrix.len()).map(|f| f),
                |x, y| matrix[x][y]
            ).style_func(&|&v| {
                    (&RGBColor((255.0*v) as u8, (255.0*v) as u8, (255.0*v) as u8)).into()
            })).unwrap();

            root.present().unwrap();
            println!("{:?}", pitch);
        }
        root.present().expect("Unable to write");
    }
    fn matrix_to_u8(matrix: &Vec<Vec<f64>>) -> Vec<Vec<u8>> {
        let mut output = vec![vec![0 as u8; matrix[0].len()]; matrix.len()];
        for x in 0..matrix.len() {
            for y in 0..matrix[x].len() {
                output[x][y] = (matrix[x][y] * 255.0) as u8;
            }
        }
        return output;

    }
    pub fn render_matrix(f_matrix: &Vec<Vec<f64>>, filename: &str) {
        let u_matrix = matrix_to_u8(f_matrix);
        let width = u_matrix.len() as u32;
        let height = u_matrix[0].len() as u32;
        let mut image = <image::ImageBuffer::<image::Rgb<u8>, _>>::new(width, height);
        for x in 0..width as usize{
            for y in 0..height as usize{
                *image.get_pixel_mut(x as u32, y as u32) = Rgb([u_matrix[x][y], u_matrix[x][y], u_matrix[x][y]]);
            }
        }
        image.save(filename).unwrap();
    }   
}