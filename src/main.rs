use minifb::{Key, KeyRepeat, Window, WindowOptions};
use num_complex::Complex;
use std::thread::sleep;
use std::time::Duration;

const SCALE: usize = 5;
const WIDTH: usize = 320 * SCALE;
const HEIGHT: usize = 200 * SCALE;

const FPS: usize = 60;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "MandelBrot Generator - NumpadPlus for more Detail, NumpadMinus for Less, Enter to Generate",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        })
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.set_target_fps(FPS);
    let mut frame:u32 = 0;
    let mut pixel:u32 = 0;
    let mut detail:u32 = 255;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        frame += 1;

        let mut update = |detail: u32| {
            for i in buffer.iter_mut() {
                pixel += 1;

                let x: u32 = pixel % WIDTH as u32;
                let y: u32 = pixel / WIDTH as u32;

                let mut x_float: f32 = x as f32 / WIDTH as f32;
                x_float = x_float * 4.0 - 2.0; // Maps to range [-2, 2]
                let mut y_float: f32 = y as f32 / HEIGHT as f32;
                y_float = y_float * 4.0 - 2.0; // Maps to range [-2, 2]
                let color: u32 = get_mandelbrot_color(x_float, y_float, detail);

                *i = color;
            }
        };;

        if window.is_key_pressed(Key::NumPadPlus, KeyRepeat::No) {
            detail *= 2;
            update(detail)
        } else if window.is_key_pressed(Key::NumPadMinus, KeyRepeat::No) && detail >= 2 {
            detail /= 2;
            update(detail)
        } else if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            update(detail)
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT);
        pixel = 0;
        sleep(Duration::from_millis(1000) / FPS as u32);
    }


}

fn get_mandelbrot_color(x:f32, y:f32, detail:u32) -> u32
{
    let scale:u32 = 16;
    let start:Complex<f64> = Complex { re:x as f64, im:y as f64};
    let mut current:Complex<f64> = Complex { re:0.0, im:0.0};


    for i in 0..detail {
        current = current*current+start;
        if current.re*current.re+current.im*current.im > 2.0 {
            return rgb_to_u32(i/scale,i,scale*i);
        }
    }

    rgb_to_u32(255,255,255)
}

fn rgb_to_u32(r:u32, g:u32, b:u32) -> u32 {
    65536 * r + 256 * g + b
}