use enable_ansi_support::enable_ansi_support;
use iter_num_tools::arange;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

const CUBE_WIDTH: f64 = 10.0;
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 55;
const BACKGROUND_ASCII_CODE: char = ' ';
const DENSITY: f64 = 0.5;
const DISTANCE_FROM_CAMERA: i32 = 70;
const K1: f64 = 100.0;

struct Scalars {
    a: f64,
    b: f64,
    c: f64,
}

struct CubeFace {
    character: char,
    cube_x: f64,
    cube_y: f64,
    cube_z: f64,
}

#[allow(clippy::redundant_field_names)]
fn main() {
    enable_ansi_support().unwrap_or_else(|e| {
        print!("ANSI support error: {}", e);
    });
    let mut scalars = Scalars {
        a: 0.0,
        b: 0.0,
        c: 0.0,
    };
    let mut calculation_duration = Duration::new(0, 0);
    let mut counter: u32 = 1;
    print!("\x1b[2J");
    loop {
        let start = Instant::now();
        let mut z_buffer = [0.0; SCREEN_WIDTH * SCREEN_HEIGHT * 4];
        let mut buffer = [BACKGROUND_ASCII_CODE; SCREEN_WIDTH * SCREEN_HEIGHT];

        for cube_x in arange(-CUBE_WIDTH..CUBE_WIDTH, DENSITY) {
            for cube_y in arange(-CUBE_WIDTH..CUBE_WIDTH, DENSITY) {
                let cube_faces = [
                    CubeFace {
                        character: '.',
                        cube_x: cube_x,
                        cube_y: cube_y,
                        cube_z: -CUBE_WIDTH,
                    },
                    CubeFace {
                        character: '$',
                        cube_x: CUBE_WIDTH,
                        cube_y: cube_y,
                        cube_z: cube_x,
                    },
                    CubeFace {
                        character: '~',
                        cube_x: -CUBE_WIDTH,
                        cube_y: cube_y,
                        cube_z: -cube_x,
                    },
                    CubeFace {
                        character: '#',
                        cube_x: -cube_x,
                        cube_y: cube_y,
                        cube_z: CUBE_WIDTH,
                    },
                    CubeFace {
                        character: ';',
                        cube_x: cube_x,
                        cube_y: -CUBE_WIDTH,
                        cube_z: -cube_y,
                    },
                    CubeFace {
                        character: '-',
                        cube_x: cube_x,
                        cube_y: CUBE_WIDTH,
                        cube_z: cube_y,
                    },
                ];

                for cube_face in &cube_faces {
                    calculate_surface(
                        cube_face.cube_x,
                        cube_face.cube_y,
                        cube_face.cube_z,
                        &scalars,
                        cube_face.character,
                        &mut z_buffer,
                        &mut buffer,
                    );
                }
            }
        }

        print!("\x1b[H");
        for (i, elem) in buffer.iter_mut().enumerate() {
            match i % SCREEN_WIDTH {
                0 => println!(),
                _ => print!("{}", elem),
            };
        }
        scalars.a -= 0.04;
        scalars.b += 0.02;
        scalars.c -= 0.04;
        println!();
        sleep(Duration::from_micros(50000));
        let frame_time = start.elapsed();
        calculation_duration = calculation_duration.checked_add(frame_time).unwrap();
        let average_duration = calculation_duration / counter;
        println!("Frame Number: {:?}", counter);
        println!("Total Time Spent Calculating: {:?}", calculation_duration);
        println!("Average Frame Time: {:?}", average_duration);
        io::stdout().flush().unwrap();
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
    let x: f64 = calculate_x(cube_x, cube_y, cube_z, scalars);
    let y: f64 = calculate_y(cube_x, cube_y, cube_z, scalars);
    let z: f64 = calculate_z(cube_x, cube_y, cube_z, scalars) + DISTANCE_FROM_CAMERA as f64;

    let ooz = 1. / z;
    let xp: i32 = ((SCREEN_WIDTH / 2) as f64 - 2. * CUBE_WIDTH + K1 * ooz * x * 2.) as i32;
    let yp: i32 = ((SCREEN_HEIGHT / 2) as f64 + K1 * ooz * y) as i32;

    let idx: i32 = xp + yp * (SCREEN_WIDTH as i32);
    if idx >= 0 && idx < (SCREEN_WIDTH * SCREEN_HEIGHT) as i32 && ooz > z_buffer[idx as usize] {
        z_buffer[idx as usize] = ooz;
        buffer[idx as usize] = character;
    };
}
