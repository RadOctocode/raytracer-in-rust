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
    let ray_origin = Tuple::set_point(1.0, 1.0, -5.0);
    let wall = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100.0;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half_wall = wall_size / 2.0;

    let mut canvas = Canvas::zero(500, 500);
    let color = color(1.0, 0.0, 0.0);

    let sphere_origin = Tuple::set_point(0.0, 0.0, 0.0);
    let test_sphere = Sphere::set_sphere(sphere_origin, 1.0);

    for y in 0..499 {
        let world_y = half_wall - pixel_size * (y as f64);
        for x in 0..499 {
            let world_x = -half_wall + pixel_size * (x as f64);
            let position = Tuple::set_point(world_x, world_y, wall);
            let new_vector = position - ray_origin.clone();
            let r = Ray::set_ray(ray_origin.clone(), new_vector.normalize());
            let xs = Ray::sphere_intersect(r, test_sphere.clone());
            if xs.len() > 0 {
                canvas.write_pixel(x, y, color.clone());
            }
        }
    }

    let name = "hello2.ppm";
    canvas.canvas_to_ppm(name).expect("bad")
}
