
pub mod utils;
pub mod graphing;
pub mod terrain;
use terrain::midpoint::midpoint_terrain::len_from_exponent;
use terrain::terrain::*;
use terrain::midpoint::midpoint_terrain;
use ndarray::Array2;
use utils::matrix_utils::*;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Settings {
    exponent: u32,
}

struct Model {
    egui: Egui,
    settings: Settings,
    terrain: Array2<f64>
}

fn model(app: &App) -> Model {
    // create / setup window
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    let starting_exponent: u32 = 3;
    let len = len_from_exponent(starting_exponent);

    Model {
        egui,
        settings: Settings {
            exponent: starting_exponent,
        },
        terrain: ndarray::Array2::<f64>::zeros((len, len))
    }
}

fn update(_app: &App, model: &mut Model, update: Update){
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Terrain Exponent: ");
        ui.add(egui::Slider::new(&mut settings.exponent, 1..=10));

        let clicked = ui.button("Regenerate").clicked();
        if clicked {
            model.terrain = midpoint_terrain::new(settings.exponent);
        }
    });
}

fn raw_window_event(app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent){
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let terrain = &model.terrain;

    let draw = app.draw();

    let len = terrain.shape()[0]-1;

    let win = app.window_rect();
    let win_width = win.right() - win.left();
    let win_height = win.top() - win.bottom();

    let pixel_width = win_width / len as f32;
    let pixel_height = win_height / len as f32;
    
    for x in 0..len{
        for y in 0..len{
            let val = terrain[[x, y]];
            let color = rgb(val, val, val);
            draw.rect()
                .height(pixel_height)
                .width(pixel_width)
                .color(color)
                .x(map_range(x, 0, len, win.left(), win.right()) + (pixel_width / 2.0))
                .y(map_range(y, len, 0, win.bottom(), win.top()) + (pixel_height / 2.0));
        }
    }
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
