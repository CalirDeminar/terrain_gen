pub mod render_2d {
    use nannou::prelude::*;
    use crate::*;
    pub fn render_2d_terrain(draw: Draw, terrain: &Array2<f64>, win: Rect, pix_w: f32, pix_h: f32) -> Draw {
        let len = terrain.shape()[0];
        utils::matrix_utils::print(&terrain);
        for x in 0..len {
            for y in 0..len{
                let val = terrain[[x, y]];
                let (nx, ny) = get_surface_normal(&terrain, x, y);
                let color = rgb(val, val, val);
                let color_inv = rgb(1.0-(val+0.1), 1.0-(val+0.1), 1.0-(val+0.1));
                
                let x_t = map_range(x, 0, len, win.left(), win.right());
                let y_t = map_range(y, len, 0, win.bottom(), win.top());
                draw.rect()
                    .height(pix_h)
                    .width(pix_w)
                    .color(color)
                    .x(x_t)
                    .y(y_t);
                // draw.text(&format!("{:.*}", 0, val*100.0))
                //     .color(color_inv)
                //     .font_size(8)
                //     .x(x_t)
                //     .y(y_t);
                
                
                // let v_x = map_range(x, 0, len, win.left(), win.right());
                // let v_y = map_range(y, len, 0, win.bottom(), win.top());
                // let p1 = pt2(v_x, v_y);
                // let p2 = pt2(v_x + (nx as f32 * pix_w * 1.5), v_y + (ny as f32 * pix_h * 1.5 ));
                
                // draw.line().start(p1).end(p2).color(color_inv).weight(1.0);
            }
        }
        return draw;
    }

    pub fn render_2d_overlay(draw: Draw, terrain: &Array2<f64>, win: Rect, font_size: text::FontSize) -> Draw {
        let len = terrain.shape()[0];
        utils::matrix_utils::print(&terrain);
        for x in 0..len {
            for y in 0..len{
                let val = terrain[[x, y]];
                let color_inv = rgb(1.0-(val+0.1), 1.0-(val+0.1), 1.0-(val+0.1));
                
                let x_t = map_range(x, 0, len, win.left(), win.right());
                let y_t = map_range(y, len, 0, win.bottom(), win.top());

                draw.text(&format!("{:.*}", 0, val*100.0))
                    .color(color_inv)
                    .font_size(font_size)
                    .x(x_t)
                    .y(y_t);
                
                
                // let v_x = map_range(x, 0, len, win.left(), win.right());
                // let v_y = map_range(y, len, 0, win.bottom(), win.top());
                // let p1 = pt2(v_x, v_y);
                // let p2 = pt2(v_x + (nx as f32 * pix_w * 1.5), v_y + (ny as f32 * pix_h * 1.5 ));
                
                // draw.line().start(p1).end(p2).color(color_inv).weight(1.0);
            }
        }
        return draw;
    }
}