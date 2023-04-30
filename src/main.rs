
pub mod utils;
pub mod graphing;
pub mod terrain;
pub mod renderer;
use terrain::erosion;
use terrain::{midpoint::midpoint_terrain::len_from_exponent, erosion_culm::erosion_culmulative
};
use terrain::midpoint::midpoint_terrain;
use ndarray::Array2;
use utils::matrix_utils::*;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

use crate::renderer::renderer::*;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(PartialEq, Clone, Debug)]
pub enum ViewModes {Terrain, Eroded, Diff}

#[derive(PartialEq, Clone, Debug)]
struct Settings {
    exponent: u32,
    culm_erosion_iterations: u32,
    iter_erosion_multipler: usize,
    mode: ViewModes,
}

pub struct Model {
    egui: Egui,
    settings: Settings,
    terrain: Array2<f64>,
    eroded_terrain: Array2<f64>,
    draw_cache: Draw,
    redraw: bool
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
        draw_cache: app.draw(),
        redraw: true
    }
}

fn update(app: &App, model: &mut Model, update: Update){
    let egui = &mut model.egui;
    let old_settings = model.settings.clone();
    let settings = &mut model.settings;

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
            model.redraw = true;
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
            model.redraw = true;
        }

        ui.separator();
        ui.heading("Iterative Erosion");
        ui.label("Iterations:");
        ui.add(egui::Slider::new(&mut settings.iter_erosion_multipler, 1..=5));
        let iter_erode = ui.button("Erode: Iter").clicked();
        if iter_erode {
            model.eroded_terrain = erosion::erosion_mod::erode(model.eroded_terrain.clone(), settings.iter_erosion_multipler);
            model.redraw = true;
        }
        if settings.mode != old_settings.mode {
            model.redraw = true;
        }
    });
    if model.redraw {
        model.draw_cache = render(app, &model.settings.mode, &model.terrain, &model.eroded_terrain);
        model.redraw = false;
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
