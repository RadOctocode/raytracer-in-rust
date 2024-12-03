mod canvas;
mod color;
mod light;
mod material;
mod matrix;
mod ray;
mod transform;
mod tuple;
use crate::canvas::Canvas;
use crate::color::color;
use crate::ray::Ray;
use crate::ray::Sphere;
use crate::tuple::Tuple;
use crate::light::PointLight;
 use crate::light::lighting;

fn main() {
    let ray_origin = Tuple::set_point(0.0, 0.0, -5.0);
    let wall = 10.0;
    //not entirely sure what this wall is suppose to represent
    let canvas_size = 100;
    let mut canvas = Canvas::zero(canvas_size, canvas_size);
    
    let light_position = Tuple::set_point(-10.0, 10.0, 8.0);
    let light_color = color(1.0, 1.0, 1.0);
    let light = PointLight::set_point_light(light_color, light_position);
    
    let sphere_origin = Tuple::set_point(0.0, 0.0, 0.0);
    let sphere_radius = 1.2;
    //as the sphere gets closer to 1.4 the sphere gets too large to render
    let mut test_sphere = Sphere::set_sphere(sphere_origin, sphere_radius);
    test_sphere.material.color = color(1.0, 2.0, 0.0);
    
    for y in 0..(canvas_size - 1) {
        let world_y = (-1.0 * (y as f64)) + (canvas_size as f64) / 2.0;
        for x in 0..(canvas_size - 1) {
            let world_x = (x as f64) - (canvas_size as f64) / 2.0;
            let position = Tuple::set_point(world_x, world_y, wall);
            let new_vector = position - ray_origin.clone();
            let r = Ray::set_ray(ray_origin.clone(), new_vector.normalize());
            let xs = Ray::sphere_intersect(&r, &test_sphere);
            
            if xs.len() > 0 {
                let hit_position = r.calculate_position(xs[0]);
                let normal = test_sphere.normal_at(hit_position.clone());
                let eye_vector = -r.direction;
                let current_color = lighting(test_sphere.material.clone(), light.clone(), hit_position.clone(), eye_vector.clone(), normal.clone());
                canvas.write_pixel(x, y, current_color.clone());
            }

        }
    }

    let name = "../examples/chapter6.ppm";
    //canvas.print_canvas();
    canvas.canvas_to_ppm(name).expect("bad")
}
