use gfs::{Canvas, Pos3, Scene, Sphere, Viewport, Light, Vec3, Surface};
use gfx_from_scratch as gfs;
use image::{RgbImage, Rgb};

const WIDTH: usize = 2000;
const HEIGHT: usize = 2000;

const LOW_WIDTH: isize = -(WIDTH as isize / 2);
const HIGH_WIDTH: isize = WIDTH as isize / 2;
const LOW_HEIGHT: isize = -(HEIGHT as isize / 2);
const HIGH_HEIGHT: isize = HEIGHT as isize / 2;

const BACKGROUND_COLOR: [u8; 3] = WHITE;

const WHITE: [u8; 3] = [255, 255, 255];
const RED: [u8; 3] = [255, 0, 0];
const GREEN: [u8; 3] = [0, 255, 0];
const BLUE: [u8; 3] = [0, 0, 255];
const YELLOW: [u8; 3] = [255, 255, 0];

fn main() {
    let viewport = Viewport::default();
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let spheres = vec![
        Sphere::new(Pos3::new(0.0, -1.0, 3.0), 1.0, RED, Surface::Shiny(500.0)),
        Sphere::new(Pos3::new(2.0, 0.0, 4.0), 1.0, BLUE, Surface::Shiny(500.0)),
        Sphere::new(Pos3::new(-2.0, 0.0, 4.0), 1.0, GREEN, Surface::Shiny(10.0)),
        Sphere::new(Pos3::new(0.0, -5001.0, 0.0), 5000.0, YELLOW, Surface::Shiny(1000.0)),
    ];

    let lights = vec![
        Light::Ambient(0.2),
        Light::Point(0.6, Pos3::new(2.0, 1.0, 0.0)),
        Light::Directional(0.2, Vec3::new(1.0, 4.0, 4.0)),
    ];

    let scene = Scene {
        spheres,
        lights,
        bg_color: BACKGROUND_COLOR,
    };


    let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);

    for x in LOW_WIDTH..HIGH_WIDTH - 1 {
        for y in LOW_HEIGHT..HIGH_HEIGHT - 1 {
            let d = canvas.to_viewport(x, y, &viewport);
            let color = scene.trace_ray(viewport.position, d, 1.0, f64::MAX);
            let (rx, ry) = canvas.topleft_rel(x, y);
            image.put_pixel(rx as u32, ry as u32, Rgb(color));
        }
    }

    image.save("./test.png").unwrap();
}
