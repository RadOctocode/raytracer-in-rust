mod color;
mod matrix;
mod tuple;
mod canvas;
use crate::color::color;
use crate::canvas::Canvas;
fn main() {
    let mut canvas = Canvas::zero(5, 3);
    let red = color(1.00, 0.00, 0.00);
    let canvas_path = "./test.ppm";
    canvas.write_pixel(0, 0, red.clone());
    canvas.canvas_to_ppm("./test.ppm".to_string());

}
