
pub mod utils;
pub mod graphing;
pub mod terrain;
use terrain::erosion;
use terrain::{midpoint::midpoint_terrain::len_from_exponent, erosion_culm::erosion_culmulative
};
use terrain::midpoint::midpoint_terrain;
use ndarray::Array2;
use utils::matrix_utils::*;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(PartialEq, Clone, Debug)]
enum ViewModes {Terrain, Eroded, Diff}

#[derive(PartialEq, Clone, Debug)]
struct Settings {
    exponent: u32,
    culm_erosion_iterations: u32,
    iter_erosion_multipler: usize,
    mode: ViewModes
}

struct Model {
    egui: Egui,
    settings: Settings,
    terrain: Array2<f64>,
    eroded_terrain: Array2<f64>,
    draw_cache: Draw
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

    let starting_exponent: u32 = 5;

    let settings = Settings {
            exponent: starting_exponent,
            culm_erosion_iterations: 1,
            iter_erosion_multipler: 1,
            mode: ViewModes::Eroded
        };
    let terrain = midpoint_terrain::new(settings.exponent);

    Model {
        egui,
        settings,
        terrain: terrain.clone(),
        eroded_terrain: terrain.clone(),
        draw_cache: app.draw()
    }
}

fn update(app: &App, model: &mut Model, update: Update){
    let egui = &mut model.egui;
    let old_settings = model.settings.clone();
    let settings = &mut model.settings;
    let mut redraw = false;

    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.heading("Terrain Generation");
        ui.label("Terrain Exponent: ");
        ui.add(egui::Slider::new(&mut settings.exponent, 1..=10));

        let regenerate = ui.button("Regenerate").clicked();
        if regenerate {
            model.terrain = midpoint_terrain::new(settings.exponent);
            model.eroded_terrain = model.terrain.clone();
            redraw = true;
        }

        ui.heading("View Mode:");
        ui.radio_value(&mut settings.mode, ViewModes::Eroded, "Eroded");
        ui.radio_value(&mut settings.mode, ViewModes::Terrain, "Origional");
        ui.radio_value(&mut settings.mode, ViewModes::Diff, "Diff");
        

        ui.separator();
        ui.heading("Culmulative Erosion");
        ui.label("Iterations:");
        ui.add(egui::Slider::new(&mut settings.culm_erosion_iterations, 2..=10));
        let culm_erode = ui.button("Erode: Culm").clicked();
        if culm_erode {
            model.eroded_terrain = erosion_culmulative::erode(model.eroded_terrain.clone(), settings.culm_erosion_iterations as usize);
            redraw = true;
        }

        ui.separator();
        ui.heading("Iterative Erosion");
        ui.label("Iterations:");
        ui.add(egui::Slider::new(&mut settings.iter_erosion_multipler, 1..=5));
        let iter_erode = ui.button("Erode: Iter").clicked();
        if iter_erode {
            model.eroded_terrain = erosion::erosion_mod::erode(model.eroded_terrain.clone(), settings.iter_erosion_multipler);
            redraw = true;
        }


        // ui.separator();
        // ui.heading("Iterative Erosion");
    });
    if redraw || settings.mode != old_settings.mode {
        let draw = app.draw();
            let base = model.terrain.clone();
        let eroded = model.eroded_terrain.clone();
        let diff = base - eroded;

        let mut working_terrain = &model.terrain;
        if settings.mode == ViewModes::Eroded {
            working_terrain = &model.eroded_terrain;
        }
        if settings.mode == ViewModes::Diff {
            working_terrain = &diff;
        }

        let len = working_terrain.shape()[0]-1;

        let win = app.window_rect();
        let win_width = win.right() - win.left();
        let win_height = win.top() - win.bottom();

        let pixel_width = win_width / len as f32;
        let pixel_height = win_height / len as f32;
        
        for x in 0..len{
            for y in 0..len{
                let val = working_terrain[[x, y]];
                let color = rgb(val, val, val);
                draw.rect()
                    .height(pixel_height)
                    .width(pixel_width)
                    .color(color)
                    .x(map_range(x, 0, len, win.left(), win.right()) + (pixel_width / 2.0))
                    .y(map_range(y, len, 0, win.bottom(), win.top()) + (pixel_height / 2.0));
            }
        }
        model.draw_cache = draw;
        println!("Rerender");
        // println!("{:?}", model.settings);
        // println!("Different?: {:?}", model.terrain != model.eroded_terrain);
        utils::matrix_utils::print(working_terrain);
    }
}

fn raw_window_event(app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent){
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = &model.draw_cache;
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
