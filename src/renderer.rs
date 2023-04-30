pub mod renderer_2d;
pub mod renderer {
    use nannou::prelude::*;
    use nannou_egui::egui::Ui;
    use crate::{*, renderer::renderer_2d::render_2d::*};

    pub fn render(app: &App, settings: Settings, terrain_data: &TerrainData) -> Draw {
        let terrain = fetch_terrain_matrix(&settings.view_mode, terrain_data);
        let len = terrain.shape()[0];

        let win = app.window_rect();
        let [pix_w, pix_h] = calc_pixel_size(&win, len);

        let mut draw = app.draw();
        draw = render_2d_terrain(draw, &terrain, win, pix_w, pix_h); 
        if settings.overlay_mode != OverlayModes::None {
            let overlay = fetch_terrain_overlay_matrix(&settings.overlay_mode, terrain_data);
            draw = render_2d_overlay(draw, &overlay, win, 8)
        }
        return draw;
    }

    pub fn draw_ui(ui: &mut Ui, settings: &mut Settings) -> (bool, bool) {
        ui.heading("Terrain Generation");
        ui.label("Terrain Exponent: ");
        ui.add(egui::Slider::new(&mut settings.exponent, 1..=10));

        let regenerate = ui.button("Regenerate").clicked();

        ui.heading("View Mode:");
        ui.radio_value(&mut settings.view_mode, ViewModes::Eroded, "Eroded");
        ui.radio_value(&mut settings.view_mode, ViewModes::Terrain, "Origional");
        ui.radio_value(&mut settings.view_mode, ViewModes::Diff, "Diff");
        ui.heading("Overlay Mode:");
        ui.radio_value(&mut settings.overlay_mode, OverlayModes::None, "Off");
        ui.radio_value(&mut settings.overlay_mode, OverlayModes::Height, "Height");
        ui.radio_value(&mut settings.overlay_mode, OverlayModes::Sediment, "Sediment");
        ui.radio_value(&mut settings.overlay_mode, OverlayModes::Water, "Water");

        ui.separator();
        ui.heading("Culmulative Erosion");
        ui.label("Iterations:");
        ui.add(egui::Slider::new(&mut settings.culm_erosion_iterations, 2..=10));
        let culm_erode = ui.button("Erode: Culm").clicked();

        return (regenerate, culm_erode);
    }

    fn calc_pixel_size(win: &Rect, len: usize) -> [f32; 2] {
        let win_width = win.right() - win.left();
        let win_height = win.top() - win.bottom();

        return [win_width / len as f32, win_height / len as f32];
    }
    fn fetch_terrain_overlay_matrix(mode: &OverlayModes, terrain: &TerrainData) -> Array2<f64> {
        match mode{
            OverlayModes::Water => return terrain.water.clone(),
            OverlayModes::Sediment => return terrain.sediment.clone(),
            OverlayModes::Height | OverlayModes::None => return terrain.terrain_base.clone()
        }
    }
    fn fetch_terrain_matrix(mode: &ViewModes, terrain: &TerrainData) -> Array2<f64> {
        match  mode{
            ViewModes::Terrain => return terrain.terrain_base.clone(),
            ViewModes::Eroded => return terrain.eroded_terrain.clone(),
            ViewModes::Diff => return terrain.terrain_base.clone() - terrain.eroded_terrain.clone()
        }
    }
}