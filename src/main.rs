mod midpoint;
mod graphing;
pub mod utils;
pub mod erosion;
use midpoint::midpoint_terrain;
use erosion::erosion_mod::erode;
use piston_window::{EventLoop, PistonWindow, WindowSettings};
use plotters::prelude::*;
use plotters_piston::{draw_piston_window};
use systemstat::platform::common::Platform;
use systemstat::System;


fn main() {
    let initial_matrix: Vec<Vec<f64>> = midpoint_terrain::new(8);
    let mut matrix = initial_matrix.clone();

    let mut window: PistonWindow = WindowSettings::new("Real Time Erosion", [500, 500])
        .samples(4)
        .build()
        .unwrap();
    let sys = System::new();
    let FPS: u32 = 10;
    window.set_max_fps(FPS as u64);

    while let Some(_) = draw_piston_window(&mut window, |b| {
        matrix = erode(matrix);
        let local_matrix = matrix.clone();
        let x_axis = 0..(local_matrix.len());
        let y_axis = 0..(local_matrix[0].len());
        let z_axis = 0.0 as f64..1.0 as f64;


        let root = b.into_drawing_area();

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .caption("Terrain", ("sans-serif", 30))
            .build_cartesian_3d(x_axis.clone(), z_axis.clone(), y_axis.clone())
            .unwrap();


        chart.with_projection(|mut p| {
            p.pitch = 1.57;
            p.scale = 0.7;
            p.into_matrix() // build the projection matrix
        });  

        chart.configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .draw()
            .unwrap();

        chart.draw_series(
            SurfaceSeries::xoz(
                (0..local_matrix.len()).map(|f| f), 
                (0..local_matrix.len()).map(|f| f), 
                |x, y| local_matrix[x][y]
            ).style_func(&|&v| {
                    (&RGBColor((255.0*v) as u8, (255.0*v) as u8, (255.0*v) as u8)).into()
            })).unwrap();
        Ok(())
    }) {}
}
