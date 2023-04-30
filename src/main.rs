
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

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ViewModes {Terrain, Eroded, Diff}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum OverlayModes {None, Height, Sediment, Water}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Settings {
    exponent: u32,
    culm_erosion_iterations: u32,
    iter_erosion_multipler: usize,
    view_mode: ViewModes,
    overlay_mode: OverlayModes
}

#[derive(PartialEq, Clone, Debug)]
pub struct TerrainData {
    terrain_base: Array2<f64>,
    eroded_terrain: Array2<f64>,
    water: Array2<f64>,
    sediment: Array2<f64>
}

pub struct Model {
    egui: Egui,
    settings: Settings,
    terrain_data: TerrainData,
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
            view_mode: ViewModes::Eroded,
            overlay_mode: OverlayModes::None
        };
    let terrain = midpoint_terrain::new(settings.exponent);
    let len = terrain.shape()[0];
    let zeros = ndarray::Array2::<f64>::zeros((len, len));

    Model {
        egui,
        settings,
        terrain_data: TerrainData {
            terrain_base: terrain.clone(),
            eroded_terrain: terrain.clone(),
            water: zeros.clone(),
            sediment: zeros.clone(),
        },
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
        let (regenerate, culm_erode) = renderer::renderer::draw_ui(ui, settings);

        if regenerate {
            model.terrain_data.terrain_base = midpoint_terrain::new(settings.exponent);
            model.terrain_data.eroded_terrain = model.terrain_data.terrain_base.clone();
            model.redraw = true;
        }

        if culm_erode {
            (model.terrain_data.eroded_terrain, 
                model.terrain_data.water, 
                model.terrain_data.sediment
            ) = erosion_culmulative::erode_debug(model.terrain_data.eroded_terrain.clone(), settings.culm_erosion_iterations as usize);
            model.redraw = true;
        }

        if settings.view_mode != old_settings.view_mode || settings.overlay_mode != old_settings.overlay_mode {
            model.redraw = true;
        }
    });
    if model.redraw {
        model.draw_cache = render(app, model.settings.clone(), &model.terrain_data);
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
