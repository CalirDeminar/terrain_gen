pub mod graphing_mod {
    use plotters::prelude::*;
    const OUT_FILE_NAME_SVG: &'static str = "E:/documents/rust/city-gen/export.svg";
    const OUT_FILE_NAME_GIF: &'static str = "E:/documents/rust/city-gen/export.gif";
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
    pub fn graph_matrix_to_gif(matrix: Vec<Vec<f64>>) {
        let root = BitMapBackend::gif(OUT_FILE_NAME_GIF, (1920, 1080), 100).unwrap().into_drawing_area();

        for pitch in (0..360).step_by(5){
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
}