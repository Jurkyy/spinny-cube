use enable_ansi_support::enable_ansi_support;
use iter_num_tools::arange;
use std::io::{self, Write};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};
const CUBE_WIDTH: f64 = 10.;
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 55;
const BACKGROUND_ASCII_CODE: char = ' ';
const SPEED: f64 = 1.0;
const DISTANCE_FROM_CAMERA: i32 = 60;
const K1: f64 = 100.;

struct Scalars {
    a: f64,
    b: f64,
    c: f64,
}

fn main() {
    match enable_ansi_support() {
        Ok(()) => {}
        Err(e) => {
            print!("ANSI support error: {}", e);
        }
    }

    let mut z_buffer: [f64; SCREEN_WIDTH * SCREEN_HEIGHT * 4];
    let mut buffer: [char; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut scalars = Scalars {
        a: 0.0,
        b: 0.0,
        c: 0.0,
    };
    io::stdout().flush().unwrap();
    let mut total_duration = Duration::new(0, 0);
    let mut _average_duration = Duration::new(0, 0);
    let mut counter: u32 = 1;
    print!("\x1b[2J");
    loop {
        let start = Instant::now();
        z_buffer = [0.; SCREEN_WIDTH * SCREEN_HEIGHT * 4];
        buffer = [BACKGROUND_ASCII_CODE; SCREEN_WIDTH * SCREEN_HEIGHT];
        for cube_x in arange(-CUBE_WIDTH..CUBE_WIDTH, SPEED) {
            for cube_y in arange(-CUBE_WIDTH..CUBE_WIDTH, SPEED) {
                calculate_surface(
                    cube_x,
                    cube_y,
                    -CUBE_WIDTH,
                    &scalars,
                    '.',
                    &mut z_buffer,
                    &mut buffer,
                );
                calculate_surface(
                    CUBE_WIDTH,
                    cube_y,
                    cube_x,
                    &scalars,
                    '$',
                    &mut z_buffer,
                    &mut buffer,
                );
                calculate_surface(
                    -CUBE_WIDTH,
                    cube_y,
                    -cube_x,
                    &scalars,
                    '~',
                    &mut z_buffer,
                    &mut buffer,
                );
                calculate_surface(
                    -cube_x,
                    cube_y,
                    CUBE_WIDTH,
                    &scalars,
                    '#',
                    &mut z_buffer,
                    &mut buffer,
                );
                calculate_surface(
                    cube_x,
                    -CUBE_WIDTH,
                    -cube_y,
                    &scalars,
                    ';',
                    &mut z_buffer,
                    &mut buffer,
                );
                calculate_surface(
                    cube_x,
                    CUBE_WIDTH,
                    cube_y,
                    &scalars,
                    '-',
                    &mut z_buffer,
                    &mut buffer,
                );
            }
        }

        print!("\x1b[H");
        let frame_time = start.elapsed();
        for (i, elem) in buffer.iter_mut().enumerate() {
            match i % SCREEN_WIDTH {
                0 => println!(),
                _ => print!("{}", elem),
            };
        }
        scalars.a += 0.05;
        scalars.b -= 0.05;
        scalars.c += 0.01;
        println!();
        total_duration = total_duration.checked_add(frame_time).unwrap();
        _average_duration = total_duration.div_f64(counter as f64);
        println!("Frame Number: {:?}", counter);
        println!("Total Time: {:?}", total_duration);
        println!("Average Frame Time: {:?}", _average_duration);
        std::io::stdout().flush().unwrap();
        sleep(Duration::from_micros(50000));
        counter += 1;
    }
}

fn calculate_x(i: f64, j: f64, k: f64, scalars: &Scalars) -> f64 {
    j * scalars.a.sin() * scalars.b.sin() * scalars.c.cos()
        - k * scalars.a.cos() * scalars.b.sin() * scalars.c.cos()
        + j * scalars.a.cos() * scalars.c.sin()
        + k * scalars.a.sin() * scalars.c.sin()
        + i * scalars.b.cos() * scalars.c.cos()
}

fn calculate_y(i: f64, j: f64, k: f64, scalars: &Scalars) -> f64 {
    j * scalars.a.cos() * scalars.c.cos() + k * scalars.a.sin() * scalars.c.cos()
        - j * scalars.a.sin() * scalars.b.sin() * scalars.c.sin()
        + k * scalars.a.cos() * scalars.b.sin() * scalars.c.sin()
        - i * scalars.b.cos() * scalars.c.sin()
}

fn calculate_z(i: f64, j: f64, k: f64, scalars: &Scalars) -> f64 {
    k * scalars.a.cos() * scalars.b.cos() - j * scalars.a.sin() * scalars.b.cos()
        + i * scalars.b.sin()
}

fn calculate_surface(
    cube_x: f64,
    cube_y: f64,
    cube_z: f64,
    scalars: &Scalars,
    character: char,
    z_buffer: &mut [f64; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
    buffer: &mut [char; SCREEN_WIDTH * SCREEN_HEIGHT],
) {
    let x = calculate_x(cube_x, cube_y, cube_z, scalars);
    let y = calculate_y(cube_x, cube_y, cube_z, scalars);
    let z = calculate_z(cube_x, cube_y, cube_z, scalars) + DISTANCE_FROM_CAMERA as f64;
    let ooz = 1. / z;
    let xp: i32 = ((SCREEN_WIDTH / 2) as f64 - 2. * CUBE_WIDTH + K1 * ooz * x * 2.) as i32;
    let yp: i32 = ((SCREEN_HEIGHT / 2) as f64 + K1 * ooz * y) as i32;

    let idx: i32 = xp + yp * (SCREEN_WIDTH as i32);
    if idx >= 0 && idx < (SCREEN_WIDTH * SCREEN_HEIGHT) as i32 && ooz > z_buffer[idx as usize] {
        z_buffer[idx as usize] = ooz;
        buffer[idx as usize] = character;
    };
}
