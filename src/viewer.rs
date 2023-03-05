pub mod terrain_viewer {
    // use ndarray::*;
    use piston_window::*;
    use ndarray::Array2;
    use std::{thread, time};

    pub fn view(incoming_matrix: &Array2<f64>) {
        let matrix = incoming_matrix;
        let opengl = OpenGL::V3_2;

        let mut window: PistonWindow = WindowSettings::new("shapes", [500, 500])
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();
        window.set_lazy(true);
        println!("{:?}", matrix.shape());
        // let limit = 10;
        let mut drew: bool = false;
        while let Some(event) = window.next() {
            println!("cycle");
            if !drew {
                window.draw_2d(&event, |context, graphics, _device| {
                    clear([1.0; 4], graphics);
                    for x in 0..matrix.shape()[0]{
                        for y in 0..matrix.shape()[1]{
                            let inten = matrix[[x, y]];
                            let col: [f32; 4] = [inten as f32, inten as f32, inten as f32, 1.0];
                            let pos: [f64; 4] = [x as f64, y as f64, (x+1)as f64, (y+1) as f64];
                            rectangle(
                                col, 
                                pos, 
                                context.transform, 
                                graphics);
                        }
                    }
                });
                drew = true;
            }

            thread::sleep(time::Duration::from_millis(1000));
        }
    }
}