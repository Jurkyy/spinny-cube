use enable_ansi_support::enable_ansi_support;
use iter_num_tools::arange;
use std::f64::consts::PI;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

const CUBE_WIDTH: f64 = 10.0;
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 55;
const BACKGROUND_ASCII_CODE: char = ' ';
const DISTANCE_FROM_CAMERA: i32 = 70;
const K1: f64 = 100.0;
const DENSITY: f64 = 0.5;

struct Scalars {
    a: f64,
    b: f64,
    c: f64,
}

// Define a trait for shapes
trait Shape {
    fn generate_points(&self) -> Vec<Point>;
}

struct Point {
    x: f64,
    y: f64,
    z: f64,
    character: char,
}

struct Cube {
    width: f64,
}

struct Sphere {
    radius: f64,
}

impl Cube {
    fn new(width: f64) -> Self {
        Cube { width }
    }
}

impl Sphere {
    fn new(radius: f64) -> Self {
        Sphere { radius }
    }
}

impl Shape for Cube {
    fn generate_points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        for cube_x in arange(-self.width..self.width, DENSITY) {
            for cube_y in arange(-self.width..self.width, DENSITY) {
                // Front face
                points.push(Point {
                    x: cube_x,
                    y: cube_y,
                    z: -self.width,
                    character: '.',
                });
                // Right face
                points.push(Point {
                    x: self.width,
                    y: cube_y,
                    z: cube_x,
                    character: '$',
                });
                // Left face
                points.push(Point {
                    x: -self.width,
                    y: cube_y,
                    z: -cube_x,
                    character: '~',
                });
                // Back face
                points.push(Point {
                    x: -cube_x,
                    y: cube_y,
                    z: self.width,
                    character: '#',
                });
                // Bottom face
                points.push(Point {
                    x: cube_x,
                    y: -self.width,
                    z: -cube_y,
                    character: ';',
                });
                // Top face
                points.push(Point {
                    x: cube_x,
                    y: self.width,
                    z: cube_y,
                    character: '-',
                });
            }
        }
        points
    }
}

impl Shape for Sphere {
    fn generate_points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        for phi in arange(0.0..PI, DENSITY / 2.0) {
            for theta in arange(0.0..2.0 * PI, DENSITY / 2.0) {
                let x = self.radius * phi.sin() * theta.cos();
                let y = self.radius * phi.sin() * theta.sin();
                let z = self.radius * phi.cos();

                points.push(Point {
                    x,
                    y,
                    z,
                    character: 'o',
                });
            }
        }
        points
    }
}

struct HexagonalPrism {
    radius: f64, // Distance from center to any vertex
    height: f64, // Height/depth of the prism
}

impl HexagonalPrism {
    fn new(radius: f64, height: f64) -> Self {
        HexagonalPrism { radius, height }
    }
}

impl Shape for HexagonalPrism {
    fn generate_points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        // Define the 6 vertices of the regular hexagon
        let angles: Vec<f64> = (0..6).map(|i| i as f64 * PI / 3.0).collect();
        let vertices: Vec<(f64, f64)> = angles
            .iter()
            .map(|&angle| (self.radius * angle.cos(), self.radius * angle.sin()))
            .collect();

        // Generate points for each face
        for i in 0..6 {
            let (x1, y1) = vertices[i];
            let (x2, y2) = vertices[(i + 1) % 6];

            // Generate points along each vertical edge
            for h in arange(-self.height / 2.0..self.height / 2.0, DENSITY) {
                points.push(Point {
                    x: x1,
                    y: y1,
                    z: h,
                    character: '|',
                });
            }

            // Generate points along the edges of top and bottom faces
            for t in arange(0.0..1.0, DENSITY) {
                let x = x1 + (x2 - x1) * t;
                let y = y1 + (y2 - y1) * t;

                // Top face edge
                points.push(Point {
                    x,
                    y,
                    z: self.height / 2.0,
                    character: '-',
                });

                // Bottom face edge
                points.push(Point {
                    x,
                    y,
                    z: -self.height / 2.0,
                    character: '-',
                });
            }

            // Fill the faces
            for r in arange(0.0..self.radius, DENSITY) {
                let steps = (2.0 * PI * r / DENSITY).ceil() as i32;
                for step in 0..steps {
                    let angle = 2.0 * PI * step as f64 / steps as f64;
                    let x = r * angle.cos();
                    let y = r * angle.sin();

                    if is_point_in_hexagon(x, y, self.radius) {
                        // Top face
                        points.push(Point {
                            x,
                            y,
                            z: self.height / 2.0,
                            character: '.',
                        });

                        // Bottom face
                        points.push(Point {
                            x,
                            y,
                            z: -self.height / 2.0,
                            character: '.',
                        });
                    }
                }
            }

            // Side faces
            for h in arange(-self.height / 2.0..self.height / 2.0, DENSITY) {
                for t in arange(0.0..1.0, DENSITY) {
                    let x = x1 + (x2 - x1) * t;
                    let y = y1 + (y2 - y1) * t;

                    points.push(Point {
                        x,
                        y,
                        z: h,
                        character: '#',
                    });
                }
            }
        }

        points
    }
}

fn is_point_in_hexagon(x: f64, y: f64, radius: f64) -> bool {
    let x = x.abs();
    let y = y.abs();
    let sqrt3 = 3.0_f64.sqrt();

    // Check if point is inside regular hexagon
    y <= sqrt3 * radius / 2.0 && y <= sqrt3 * (radius - x)
}

struct TwistedTorus {
    major_radius: f64, // Distance from center of tube to center of torus
    minor_radius: f64, // Radius of the tube
    twist_factor: f64, // How much twist to apply
    time: f64,         // For animation
}

impl TwistedTorus {
    fn new(major_radius: f64, minor_radius: f64) -> Self {
        TwistedTorus {
            major_radius,
            minor_radius,
            twist_factor: 2.0,
            time: 0.0,
        }
    }

    fn update(&mut self, delta_time: f64) {
        self.time += delta_time;
        self.twist_factor = 2.0 + (self.time * 0.5).sin();
    }
}

impl Shape for TwistedTorus {
    fn generate_points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        // Parameters for detail level
        let u_steps = (self.major_radius * 15.0) as i32;
        let v_steps = (self.minor_radius * 15.0) as i32;

        // Generate points around the torus
        for u_step in 0..u_steps {
            let u = 2.0 * PI * (u_step as f64) / (u_steps as f64);

            for v_step in 0..v_steps {
                let v = 2.0 * PI * (v_step as f64) / (v_steps as f64);

                // Calculate the twist offset
                let twist = self.twist_factor * u;

                // Basic torus coordinates
                let x = (self.major_radius + self.minor_radius * v.cos()) * u.cos();
                let y = (self.major_radius + self.minor_radius * v.cos()) * u.sin();
                let z = self.minor_radius * v.sin();

                // Apply twist transformation
                let twisted_x = x * (twist.cos()) - y * (twist.sin());
                let twisted_y = x * (twist.sin()) + y * (twist.cos());
                let twisted_z = z;

                // Choose character based on position
                let character = match (v_step % 3, u_step % 2) {
                    (0, 0) => '.',
                    (0, _) => '|',
                    (1, 0) => '○',
                    (1, _) => '&',
                    (_, 0) => '-',
                    (_, _) => '#',
                };

                points.push(Point {
                    x: twisted_x,
                    y: twisted_y,
                    z: twisted_z,
                    character,
                });
            }
        }

        points
    }
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

    let cube = Cube::new(10.0);
    let sphere = Sphere::new(10.0);
    let hexagon = HexagonalPrism::new(10.0, 20.0);
    let mut twisted_torus = TwistedTorus::new(15.0, 5.0);

    // Create an enum to handle the different shape types
    enum ShapeType<'a> {
        Static(&'a dyn Shape),
        Animated(&'a mut TwistedTorus),
    }

    let mut shapes: Vec<ShapeType> = vec![
        ShapeType::Animated(&mut twisted_torus),
        ShapeType::Static(&cube),
        ShapeType::Static(&sphere),
        ShapeType::Static(&hexagon),
    ];

    let mut shape_index = 0;
    let switch_interval = Duration::from_secs(10); // Switch every 10 seconds
    let mut last_switch = Instant::now();

    let mut calculation_duration = Duration::new(0, 0);
    let mut counter: u32 = 1;
    print!("\x1b[2J");

    loop {
        let start = Instant::now();
        if start.duration_since(last_switch) >= switch_interval {
            shape_index = (shape_index + 1) % shapes.len();
            last_switch = start;
        }

        // Update the torus if it's the current shape
        if let ShapeType::Animated(torus) = &mut shapes[0] {
            torus.update(0.011);
        }

        let mut z_buffer = [0.0; SCREEN_WIDTH * SCREEN_HEIGHT * 4];
        let mut buffer = [BACKGROUND_ASCII_CODE; SCREEN_WIDTH * SCREEN_HEIGHT];

        // Generate and render points for the selected shape
        let points = match &shapes[shape_index] {
            ShapeType::Static(shape) => shape.generate_points(),
            ShapeType::Animated(torus) => torus.generate_points(),
        };

        for point in points {
            calculate_surface(
                point.x,
                point.y,
                point.z,
                &scalars,
                point.character,
                &mut z_buffer,
                &mut buffer,
            );
        }

        print!("\x1b[H");
        for (i, elem) in buffer.iter_mut().enumerate() {
            match i % SCREEN_WIDTH {
                0 => println!(),
                _ => print!("{}", elem),
            };
        }

        scalars.a -= 0.03;
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
