mod canvas;
mod color;
mod matrix;
mod ray;
mod transform;
mod tuple;
use crate::canvas::Canvas;
use crate::color::color;
use crate::ray::Ray;
use crate::ray::Sphere;
use crate::tuple::Tuple;

fn main() {
    let ray_origin = Tuple::set_point(0.0, 0.0, -5.0);
    let wall = 10.0;
    //not entirely sure what this wall is suppose to represent
    let canvas_size = 100;
    let mut canvas = Canvas::zero(canvas_size, canvas_size);
    let color = color(1.0, 0.0, 0.0);

    let sphere_origin = Tuple::set_point(0.0, 0.0, 0.0);
    let sphere_radius = 1.3;
    //as the sphere gets closer to 1.4 the sphere gets too large to render
    let test_sphere = Sphere::set_sphere(sphere_origin, 1.3);
    for y in 0..(canvas_size - 1) {
        let world_y = (-1.0 * (y as f64)) + (canvas_size as f64) / 2.0;
        for x in 0..(canvas_size - 1) {
            let world_x = (x as f64) - (canvas_size as f64) / 2.0;
            let position = Tuple::set_point(world_x, world_y, wall);
            let new_vector = position - ray_origin.clone();
            let r = Ray::set_ray(ray_origin.clone(), new_vector.normalize());
            let xs = Ray::sphere_intersect(&r, &test_sphere);
            if xs.len() > 0 {
                canvas.write_pixel(x, y, color.clone());
            }
        }
    }

    let name = "../examples/chapter5.ppm";
    //canvas.print_canvas();
    canvas.canvas_to_ppm(name).expect("bad")
}
