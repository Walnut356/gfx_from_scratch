use std::{sync::Arc, sync::Mutex, time::Instant};

use image::{Rgb, RgbImage};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use raster::prelude::*;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

const LOW_WIDTH: isize = -(WIDTH as isize / 2);
const HIGH_WIDTH: isize = WIDTH as isize / 2;
const LOW_HEIGHT: isize = -(HEIGHT as isize / 2);
const HIGH_HEIGHT: isize = HEIGHT as isize / 2;

const BACKGROUND_COLOR: [u8; 3] = [0, 0, 0];

fn main() {
    let mut canvas = Canvas::new(500, 500);

    let now = Instant::now();

    canvas.draw_line(p![50, 50], p![300, 60], GRAY);

    let dur = now.elapsed();

    println!("{dur:?}");

    canvas.save("test");
}
