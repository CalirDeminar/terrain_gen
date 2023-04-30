pub mod renderer {
    use nannou::prelude::*;
    use crate::*;
    pub fn render(app: &App, mode: &ViewModes, terrain: &Array2<f64>, eroded_terrain: &Array2<f64>) -> Draw {
        let draw = app.draw();
        let terrain = fetch_terrain_matrix(mode, terrain, eroded_terrain);
        let len = terrain.shape()[0];

        let win = app.window_rect();
        let [pix_w, pix_h] = calc_pixel_size(&win, len);
        for x in 0..len {
            for y in 0..len{
                let val = terrain[[x, y]];
                let (nx, ny) = get_surface_normal(&terrain, x, y);
                let color = rgb(val, val, val);
                let color_inv = rgb(1.0-(val+0.1), 1.0-(val+0.1), 1.0-(val+0.1));
                
                draw.rect()
                    .height(pix_h)
                    .width(pix_w)
                    .color(color)
                    .x(map_range(x, 0, len, win.left(), win.right()))
                    .y(map_range(y, len, 0, win.bottom(), win.top()));
                
                
                // let v_x = map_range(x, 0, len, win.left(), win.right());
                // let v_y = map_range(y, len, 0, win.bottom(), win.top());
                // let p1 = pt2(v_x, v_y);
                // let p2 = pt2(v_x + (nx as f32 * pix_w * 1.5), v_y + (ny as f32 * pix_h * 1.5 ));
                
                // draw.line().start(p1).end(p2).color(color_inv).weight(1.0);
            }
        }
            
        println!("Rerender");
        // println!("{:?}", model.settings);
        // println!("Different?: {:?}", model.terrain != model.eroded_terrain);
        utils::matrix_utils::print(&terrain);
        
        return draw;    
    }
    fn calc_pixel_size(win: &Rect, len: usize) -> [f32; 2] {
        let win_width = win.right() - win.left();
        let win_height = win.top() - win.bottom();

        return [win_width / len as f32, win_height / len as f32];

    }
    fn fetch_terrain_matrix(mode: &ViewModes, terrain: &Array2<f64>, eroded_terrain: &Array2<f64>) -> Array2<f64> {
        match  mode{
            ViewModes::Terrain => return terrain.clone(),
            ViewModes::Eroded => return eroded_terrain.clone(),
            ViewModes::Diff => return terrain.clone() - eroded_terrain.clone()
        }
    }
}