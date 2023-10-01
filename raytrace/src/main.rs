use std::{sync::Mutex, time::Instant, sync::Arc};

use rt::{ Light, Pos3, Matrix, Scene, objects::Sphere, Surface, Vec3, Viewport, identity_matrix, topleft_rel};
use raytrace as rt;
use image::{Rgb, RgbImage};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

const LOW_WIDTH: isize = -(WIDTH as isize / 2);
const HIGH_WIDTH: isize = WIDTH as isize / 2;
const LOW_HEIGHT: isize = -(HEIGHT as isize / 2);
const HIGH_HEIGHT: isize = HEIGHT as isize / 2;

const BACKGROUND_COLOR: [u8; 3] = BLACK;

const WHITE: [u8; 3] = [255, 255, 255];
const BLACK: [u8; 3] = [0, 0, 0];
const RED: [u8; 3] = [255, 0, 0];
const GREEN: [u8; 3] = [0, 255, 0];
const BLUE: [u8; 3] = [0, 0, 255];
const YELLOW: [u8; 3] = [255, 255, 0];
const CYAN: [u8; 3] = [0, 255, 255];
const SKY_BLUE: [u8;3] = [135, 206, 255];

fn main() {
//     let canvas = Canvas::new(WIDTH, HEIGHT);

    let identity = identity_matrix!();

    let spheres = vec![
        Arc::new(Sphere::new(
            Matrix::skew(1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            RED,
            Surface::Shiny(500.0),
            0.2,
        )),];

    let scene = Scene {
        spheres,
        lights: Vec::new(),
        bg_color: BACKGROUND_COLOR,
    };


    let image = Mutex::new(RgbImage::new(WIDTH as u32, HEIGHT as u32));

    let viewport = Viewport::new(Pos3::new(0.0, 0.0, -5.0), 1.0, 1.0);

    let now = Instant::now();

    // pretty lazy way to do it, the mutex will slow it down a lot
    (LOW_WIDTH..HIGH_WIDTH - 1).into_par_iter().for_each(|x| {
        for y in LOW_HEIGHT..HIGH_HEIGHT - 1 {
            let d = viewport.ray_from_coord(x, y, WIDTH, HEIGHT);
            let color = scene.trace_ray(d, 1.0, f32::MAX, 3);
            let (rx, ry) = topleft_rel(WIDTH, HEIGHT, x, y);
            image
                .lock()
                .unwrap()
                .put_pixel(rx as u32, ry as u32, Rgb(color));
        }
    });

    let dur = now.elapsed();

    println!("Time to trace rays: {:?}", dur);

    image.lock().unwrap().save("./test.png").unwrap();
}
