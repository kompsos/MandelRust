use minifb::MouseButton;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use num_complex::Complex;
use std::thread::sleep;
use std::time::Duration;

const SCALE: usize = 8;
const WIDTH: usize = 320 * SCALE;
const HEIGHT: usize = 200 * SCALE;
const FPS: usize = 60;

const ZOOM_FACTOR: f32 = 1.5; // Factor by which zoom happens
const MOVE_FACTOR: f32 = 0.1; // How much the center moves per key press

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut detail: u32 = 255;

    // Center of the fractal view and zoom level
    let mut center_x: f32 = 0.0;
    let mut center_y: f32 = 0.0;
    let mut zoom: f32 = 2.0; // Default zoom level (2.0 means it shows [-2, 2])

    let mut window = Window::new(
        "Mandelbrot Generator - NumpadPlus for more Detail, NumpadMinus for Less, Enter to Generate",
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

    let mut update_window = |buffer: &mut Vec<u32>, window: &mut Window| {
        let mut pixel: u32 = 0;

        let mut generate = |details: u32, &_center_y: &f32, &_center_x: &f32, &_zoom: &f32| {
            for i in buffer.iter_mut() {
                pixel += 1;
                *i = get_pixel(&_center_x, &_center_y, &_zoom, details, pixel);
            }
        };

        // Handle user input
        if window.is_key_pressed(Key::NumPadPlus, KeyRepeat::No) {
            detail *= 2;
            generate(detail, &center_y, &center_x, &zoom);
        } else if window.is_key_pressed(Key::NumPadMinus, KeyRepeat::No) && detail >= 2 {
            detail /= 2;
            generate(detail, &center_y, &center_x, &zoom);
        } else if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            generate(detail, &center_y, &center_x, &zoom);
        }

        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            if window.get_mouse_down(minifb::MouseButton::Left) {
                // Map the mouse click (x, y) to complex coordinates in the fractal
                let mouse_x = (x as f32 / WIDTH as f32) * 4.0 - 2.0; // Map to [-2, 2]
                let mouse_y = (y as f32 / HEIGHT as f32) * 4.0 - 2.0; // Map to [-2, 2]

                // Adjust the center shift based on the zoom level
                let zoom_factor = 1.0 / zoom; // Inverse of zoom level, so smaller zoom = larger shift

                // Update the center with a smaller jump as you zoom in
                center_x = center_x + (mouse_x - center_x) * zoom_factor;
                center_y = center_y + (mouse_y - center_y) * zoom_factor;

                // Regenerate the fractal with the new center and zoom
                generate(detail, &center_y, &center_x, &zoom);
            }
        }

        if window.is_key_pressed(Key::PageUp, KeyRepeat::No) {
            zoom *= ZOOM_FACTOR; // Zoom in
            generate(detail, &center_y, &center_x, &zoom);
        }
        if window.is_key_pressed(Key::PageDown, KeyRepeat::No) {
            if(zoom <= 2.0) {return};
            zoom /= ZOOM_FACTOR; // Zoom out
            generate(detail, &center_y, &center_x, &zoom);
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        update_window(&mut buffer, &mut window);
        sleep(Duration::from_millis(1000) / FPS as u32);
    }
}

fn get_pixel(&center_x: &f32, &center_y: &f32, &zoom: &f32, detail: u32, pixel: u32) -> u32 {
    let x: u32 = pixel % WIDTH as u32;
    let y: u32 = pixel / WIDTH as u32;

    // Map pixel coordinates to fractal coordinates
    let mut x_float: f32 = x as f32 / (WIDTH as f32);
    x_float = x_float * 4.0 - 2.0; // Range [-2, 2]

    let mut y_float: f32 = y as f32 / (HEIGHT as f32);
    y_float = y_float * 4.0 - 2.0; // Range [-2, 2]

    // Apply zoom and center shift
    x_float = center_x + x_float / zoom;
    y_float = center_y + y_float / zoom;

    let color: u32 = get_mandelbrot_color(x_float, y_float, detail, 16);
    color
}

fn get_mandelbrot_color(x: f32, y: f32, detail: u32, scale: u32) -> u32 {
    let start: Complex<f64> = Complex { re: x as f64, im: y as f64 };
    let mut current: Complex<f64> = Complex { re: 0.0, im: 0.0 };

    for i in 0..detail {
        current = current * current + start;
        if current.re * current.re + current.im * current.im > 32.0 {
            return rgb_to_u32(i * scale, i, scale / i);
        }
    }

    rgb_to_u32(255, 255, 255)
}

fn rgb_to_u32(r: u32, g: u32, b: u32) -> u32 {
    65536 * r + 256 * g + b
}
