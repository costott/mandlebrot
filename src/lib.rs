use macroquad::prelude::*;
use chrono::format::strftime::StrftimeItems;
use std::sync::{Arc, Mutex};
use std::thread;
use std::f64::consts::PI;
use std::slice::Iter;
use std::fs;
use ::rand::Rng;
use threadpool::ThreadPool;

pub mod complex;
use complex::Complex;
use complex::BigComplex;
pub mod palletes;
pub mod layers;
use layers::{Layers, LayerType};

// width+height only used for the buhddabrot
pub const WIDTH: usize = 600;
pub const HEIGHT: usize = 600;

// both are techically the actual value squared
pub const BAILOUT: f64 = 4f64;
pub const EPSILON: f64 = 1e-6f64;

// 3D 
pub const H2: f64 = 1.5;
pub const ANGLE: f64 = -45.;
pub const BAILOUT_3D: f64 = 10000.;

// orbit trap
pub const BAILOUT_ORBIT_TRAP: f64 = 50.0;

// user changing view
pub const ZOOM_PERCENT_INC: f64 = 0.5f64;
pub const MAX_ITER_INC_SPEED: f32 = 10f32;
pub const PALLETE_LENGTH_INC_SPEED: f32 = 50f32;

pub const THREADS: usize = 15; //12 14 15 17

pub const START_PALLETE_LENGTH: f32 = 153.173;//250.;
pub const START_PALLETE_2_LENGTH: f32 = 18.0;

pub const MAX_ITERATION_STEPS: [f32; 5] = [250.0, 500.0, 1000.0, 2500.0, 5000.0];

#[derive(Clone)]
pub struct JuliaSeed {
    real: f64,
    im: f64
}
impl JuliaSeed {
    pub fn new(real: f64, im: f64) -> JuliaSeed {
        JuliaSeed { real, im }
    }
}

pub mod orbit_trap {
    use crate::BAILOUT_ORBIT_TRAP;

    use super::complex::Complex;

    #[derive(Clone, Copy)]
    pub struct OrbitTrapPoint {
        point: Complex
    }
    impl OrbitTrapPoint {
        pub fn new(point: (f64, f64)) -> OrbitTrapPoint {
            OrbitTrapPoint { point: Complex::new(point.0, point.1) }
        }

        /// returns the distance squared between the 
        /// given complex number and the point trap
        pub fn distance2(&self, z: Complex) -> f64 {
            (z-self.point).abs_squared()
        }

        /// returns the maximum possible distance
        /// a complex number can be from the trap
        pub fn greatest_distance2(&self) -> f64 {
            (BAILOUT_ORBIT_TRAP.sqrt() + self.point.abs_squared().sqrt()).powi(2)
        }
    }

    #[derive(Clone, Copy)]
    pub struct OrbitTrapCross {
        centre: Complex,
        arm_length: f64
    }
    impl OrbitTrapCross {
        pub fn new(centre: (f64, f64), arm_length: f64) -> OrbitTrapCross {
            OrbitTrapCross { 
                centre: Complex::new(centre.0, centre.1), 
                arm_length
            }
        }
        
        /// returns the shortest distance squared between the 
        /// given complex number and the cross trap
        pub fn distance2(&self, z: Complex) -> f64 {
            let shortest_distance;
            let x_dist = z.real-self.centre.real;
            let x_dist2 = x_dist.powi(2);
            let y_dist = z.im-self.centre.im;
            let y_dist2 = y_dist.powi(2);
            if self.centre.im - self.arm_length <= z.im && z.im <= self.centre.im + self.arm_length &&
                self.centre.real - self.arm_length <= z.real && z.real <= self.centre.real + self.arm_length {
                shortest_distance = f64::min(x_dist2, y_dist2);
            } else if x_dist2 < y_dist2 {
                shortest_distance = (z-
                    Complex::new(self.centre.real, self.centre.im+y_dist.signum()*self.arm_length)
                ).abs_squared();
            } else {
                shortest_distance = (z-
                    Complex::new(self.centre.real+x_dist.signum()*self.arm_length, self.centre.im)
                ).abs_squared();
            }

            shortest_distance
        }

        /// returns the maximum possible distance
        /// a complex number can be from the trap
        pub fn greatest_distance2(&self) -> f64 {
            (BAILOUT_ORBIT_TRAP.sqrt() + self.centre.abs_squared().sqrt()).powi(2)
        }
    }

    #[derive(Clone, Copy)]
    pub struct OrbitTrapCircle {
        centre: Complex,
        pub radius: f64
    }
    impl OrbitTrapCircle {
        pub fn new(centre: (f64, f64), radius: f64) -> OrbitTrapCircle {
            OrbitTrapCircle { 
                centre: Complex::new(centre.0, centre.1), 
                radius
            }
        }

        /// returns the shortest distance between the 
        /// given complex number and the circle trap
        pub fn distance2(&self, z: Complex) -> f64 {
            ((z-self.centre).abs_squared().sqrt() - self.radius).powi(2)
        }

        /// returns the maximum possible distance
        /// a complex number can be from the trap
        pub fn greatest_distance2(&self) -> f64 {
            let big_rad = BAILOUT_ORBIT_TRAP.sqrt();
            f64::max(
                big_rad - (self.radius - self.centre.abs_squared().sqrt()), 
                self.radius
            ).powi(2)
        }

        /// minimum possible distance a point can be from the trap
        pub fn minimum_distance2(&self) -> f64 {
            // smallest distance between radius of trap and bailout
            let circle_distance = (self.radius - self.centre.abs_squared().sqrt()) - BAILOUT_ORBIT_TRAP.sqrt();
            f64::max(0.0, circle_distance).powi(2)
        }
    }

    #[derive(Clone, Copy)]
    pub enum OrbitTrapType {
        Point(OrbitTrapPoint),
        Cross(OrbitTrapCross),
        Circle(OrbitTrapCircle)
    }
    impl OrbitTrapType {
        /// returns the greatest possible distance squared of a point to the trap
        pub fn greatest_distance2(&self) -> f64 {
            match self {
                OrbitTrapType::Point(point) => point.greatest_distance2(),
                OrbitTrapType::Cross(cross) => cross.greatest_distance2(),
                OrbitTrapType::Circle(circle) => circle.greatest_distance2()
            }
        }
        
        /// returns the distance squared between the given complex number and trap
        pub fn distance2(&self, z: Complex) -> f64 {
            match self {
                OrbitTrapType::Point(point) => point.distance2(z),
                OrbitTrapType::Cross(cross) => cross.distance2(z),
                OrbitTrapType::Circle(circle) => circle.distance2(z)
            }
        }
    }
}

use orbit_trap::*;

#[derive(Clone)]
pub struct ScreenDimensions {
    x: usize,
    y: usize
}
#[allow(unused)]
impl ScreenDimensions {
    pub fn new(x: usize, y: usize) -> ScreenDimensions {
        ScreenDimensions { x, y }
    }

    pub fn from_tuple(dimensions: (usize, usize)) -> ScreenDimensions {
        ScreenDimensions { x: dimensions.0, y: dimensions.1 }
    }

    pub fn tuple_8k() -> (usize, usize) {
        (7680, 4320)
    }

    pub fn tuple_4k() -> (usize, usize) {
        (3840, 2160)
    }

    pub fn tuple_1080p() -> (usize, usize) {
        (1920, 1080)
    }

    /// returns a string representing the dimension
    fn as_string(&self) -> String {
        let res = format!["{}x{}", self.x, self.y];
        String::from( match (self.x, self.y) {
            (1920, 1080) => "1080p",
            (3840, 2160) => "4k",
            (7680, 4320) => "8k",
            (_, _) => &res
        })
    }
}

/// returns the number of steps it took to diverge
/// (0 if it didn't escape)
fn diverges(c: Complex, max_iterations: u32) -> f64 {
    let mut z = c;
    for i in 0..max_iterations {
        if z.abs_squared() > BAILOUT {
            let log_zmod = f64::log2(z.abs_squared()) / 2.0;
            let nu = f64::log2(log_zmod);
            let smooth_iteration = i as f64 + 1.0 - nu;
            return smooth_iteration;
        }  
        z = z.square() + c;
    }
    0.0
}

// DERIVATIVED AND 3D DIVERGENCE THEORY FROM:
// https://www.math.univ-toulouse.fr/~cheritat/wiki-draw/index.php/Mandelbrot_set#Drawing_algorithms

/// derivatived divergence
fn diverges_der(c: Complex, max_iterations: u32) -> f64 {
    let mut z = c;
    let mut der = Complex::new(1., 0.);
    for i in 0..max_iterations {
        if der.abs_squared() < EPSILON {
            return 0.;
        }
        if z.abs_squared() > BAILOUT {
            let log_zmod = f64::log2(z.abs_squared()) / 2.0;
            let nu = f64::log2(log_zmod);
            let smooth_iteration = i as f64 + 1.0 - nu;
            return smooth_iteration;
        }  
        der = der * (z * 2.); // brackets not needed but just to make more sense
        z = z.square() + c;
    }
    0.0
}

/// 3d divergence
fn diverges_3d(c: Complex, max_iterations: u32) -> f64 {
    let v = Complex::new(
        f64::cos(ANGLE * (PI / 180.)),
        f64::sin(ANGLE * (PI / 180.))
    );

    let mut z = c;
    let dc = Complex::new(1., 0.);
    let mut der = dc;
    for _ in 0..max_iterations {
        if der.abs_squared() < EPSILON {
            return 0.;
        }
        if z.abs_squared() > BAILOUT_3D {
            let mut u = z / der;
            u = u / f64::sqrt(u.abs_squared());
            let mut t = u.real*v.real + u.im*v.im + H2;
            t = t/(1.+H2);
            if t < 0. {t = 0.};
            return t;
        }  
        der = der * (z * 2.) + dc; // brackets not needed but just to make more sense
        z = z.square() + c;
    }
    0.0
}


/// 3d divergence, returning t and (smooth) iteration
fn diverges_3d_coloured(c: Complex, max_iterations: u32) -> (f64, f64) {
    let v = Complex::new(
        f64::cos(ANGLE * (PI / 180.)),
        f64::sin(ANGLE * (PI / 180.))
    );

    let mut z = c;
    let dc = Complex::new(1., 0.);
    let mut der = dc;
    for i in 0..max_iterations {
        // UNCOMMENT FOR:
        // - PERFORMANCE INCREASE FOR POINTS IN THE SET
        // - PERFORMANCE DECREASE (SLIGHTLY) FOR POINTS OUTSIDE THE SET
        // if der.abs_squared() < EPSILON {
        //     // return i as f64;
        //     return (0., 0.);
        // }
        if z.abs_squared() > BAILOUT_3D {
            let mut u = z / der;
            u = u / f64::sqrt(u.abs_squared());
            let mut t = u.real*v.real + u.im*v.im + H2;
            t = t/(1.+H2);
            if t < 0. {t = 0.}; 
            let log_zmod = f64::log2(z.abs_squared()) / 2.0;
            let nu = f64::log2(log_zmod);
            let smooth_iteration = i as f64 + 1.0 - nu;
            return (t, smooth_iteration);
        }  
        der = der * (z * 2.) + dc; // brackets not needed but just to make more sense
        z = z.square() + c;
    }
    (0.0, 0.0)
}

/// 3d divergence
fn diverges_3d_variated(c: Complex, max_iterations: u32) -> f64 {
    let v = Complex::new(
        f64::cos(ANGLE * (PI / 180.)),
        f64::sin(ANGLE * (PI / 180.))
    );

    let mut z = c;
    let dc = Complex::new(1., 0.);
    let mut der = dc;
    let mut der2 = Complex::new(0., 0.);
    for _ in 0..max_iterations {
        if der.abs_squared() < EPSILON {
            return 0.;
        }
        if z.abs_squared() > BAILOUT_3D {
            let lo = 0.5 * f64::log10(z.abs_squared());
            let mut u = z * der * (
                (der.square()).conjugate()*(1.+lo) 
                - (z * der2).conjugate() * lo
            );
            u = u / f64::sqrt(u.abs_squared());
            let mut t = u.real*v.real + u.im*v.im + H2;
            t = t/(1.+H2);
            if t < 0. {t = 0.};
            return t;
        }  
        der2 = (z*der2+der.square())*2.;
        der = der * (z * 2.) + dc; // brackets not needed but just to make more sense
        z = z.square() + c;
    }
    0.0
}

/// orbit trap colouring
fn diverges_orbit_trap(c: Complex, max_iterations: u32, trap: &OrbitTrapType) -> f64 {
    let mut min_trap_distance2 = match trap {
        OrbitTrapType::Point(point) => point.greatest_distance2(),
        OrbitTrapType::Cross(cross) => cross.greatest_distance2(),
        OrbitTrapType::Circle(circle) => circle.greatest_distance2()
    };
    let divisor = min_trap_distance2.sqrt() / max_iterations as f64;
    let mut z = c;

    for _ in 0..max_iterations {
        let z_trap_distance2 = match trap {
            OrbitTrapType::Point(point) => point.distance2(z),
            OrbitTrapType::Cross(cross) => cross.distance2(z),
            OrbitTrapType::Circle(circle) => circle.distance2(z)
        };
        if z_trap_distance2 < min_trap_distance2 {
            min_trap_distance2 = z_trap_distance2;
        }
        if z.abs_squared() > BAILOUT_ORBIT_TRAP {
            // convert min trap distance as if working with max iterations
            return min_trap_distance2.sqrt() / divisor;
        }  
        z = z.square() + c;
    }
    
    0.0
}

/// orbit trap colouring, returning t and the trapped i
fn diverges_orbit_trap_3d(c: Complex, max_iterations: u32, trap: &OrbitTrapType) -> (f64, f64) {
    let v = Complex::new(
        f64::cos(ANGLE * (PI / 180.)),
        f64::sin(ANGLE * (PI / 180.))
    );
    let mut min_trap_distance2 = match trap {
        OrbitTrapType::Point(point) => point.greatest_distance2(),
        OrbitTrapType::Cross(cross) => cross.greatest_distance2(),
        OrbitTrapType::Circle(circle) => circle.greatest_distance2()
    };
    let divisor = min_trap_distance2.sqrt() / max_iterations as f64;
    let mut z = c;
    let dc = Complex::new(1., 0.);
    let mut der = dc;
    
    for _ in 0..max_iterations {
        let z_trap_distance2 = match trap {
            OrbitTrapType::Point(point) => point.distance2(z),
            OrbitTrapType::Cross(cross) => cross.distance2(z),
            OrbitTrapType::Circle(circle) => circle.distance2(z)
        };
        if z_trap_distance2 < min_trap_distance2 {
            min_trap_distance2 = z_trap_distance2;
        }
        if z.abs_squared() > BAILOUT_ORBIT_TRAP {
            let mut u = z / der;
            u = u / f64::sqrt(u.abs_squared());
            let mut t = u.real*v.real + u.im*v.im + H2;
            t = t/(1.+H2);
            if t < 0. {t = 0.}; 

            // convert min trap distance as if working with max iterations
            // min_trap_distance2.sqrt() / divisor
            let trapped_i =  min_trap_distance2.sqrt() / divisor;
            return (t, trapped_i)
        }  
        der = der * (z * 2.) + dc; // brackets not needed but just to make more sense
        z = z.square() + c;
    }
    (0.0, 0.0)
}

/// orbit trap colouring, returning t, trapped i, and smooth i
fn diverges_orbit_trap_3d_coloured(c: Complex, max_iterations: u32, trap: &OrbitTrapType) -> (f64, f64, f64) {
    let v = Complex::new(
        f64::cos(ANGLE * (PI / 180.)),
        f64::sin(ANGLE * (PI / 180.))
    );
    let mut min_trap_distance2 = trap.greatest_distance2();
    let divisor = min_trap_distance2.sqrt() / max_iterations as f64;
    let mut z = c;
    let dc = Complex::new(1., 0.);
    let mut der = dc;
    
    for i in 0..max_iterations {
        let z_trap_distance2 = trap.distance2(z);
        if z_trap_distance2 < min_trap_distance2 {
            min_trap_distance2 = z_trap_distance2;
        }
        if z.abs_squared() > BAILOUT_ORBIT_TRAP {
            let mut u = z / der;
            u = u / f64::sqrt(u.abs_squared());
            let mut t = u.real*v.real + u.im*v.im + H2;
            t = t/(1.+H2);
            if t < 0. {t = 0.}; 

            // convert min trap distance as if working with max iterations
            // min_trap_distance2.sqrt() / divisor
            let trapped_i =  min_trap_distance2.sqrt() / divisor;

            let log_zmod = f64::log2(z.abs_squared()) / 2.0;
            let nu = f64::log2(log_zmod);
            let smooth_iteration = i as f64 + 1.0 - nu;

            return (t, trapped_i, smooth_iteration)
        }  
        der = der * (z * 2.) + dc; // brackets not needed but just to make more sense
        z = z.square() + c;
    }
    (0.0, 0.0, 0.0)
}

// const SEED: Complex = Complex { real: 0.285 , im: 0. };
// const SEED: Complex = Complex { real: -0.4, im: 0.6 };
/// julia set divergence
fn diverges_julia(c: Complex, max_iterations: u32, seed: &JuliaSeed) -> f64 {
    let seed = Complex::new(seed.real, seed.im);

    let mut z = c;
    for i in 0..max_iterations {
        z = z.square() + seed;
        if z.abs_squared() > BAILOUT {
            let log_zmod = f64::log2(z.abs_squared()) / 2.0;
            let smooth_iteration = i as f64 - f64::log2(f64::max(1.0, log_zmod));
            return smooth_iteration
        }
    }
    0.0
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1f32 - t) * a + t * b
}

fn interpolate_colour(c1: Color, c2: Color, fraction: f32) -> Color {
    Color::new(
        lerp(c1.r, c2.r, fraction),
        lerp(c1.g, c2.g, fraction),
        lerp(c1.b, c2.b, fraction),
        1.0
    )
}

fn escape_time(diverge_num: f64, pallete: &Vec<Color>) -> Color {
    if diverge_num == 0. {
        // return COLOUR_MAP[COLOUR_MAP.len()-1];
        return BLACK;
    }

    let lower_colour = pallete[diverge_num.floor() as usize];
    let upper_colour = pallete[(diverge_num.floor() as usize + 1).min(pallete.len()-1)];
    
    interpolate_colour(lower_colour, upper_colour, diverge_num as f32 % 1.0)
}

fn colour_3d(t: f64, colour: Color) -> Color {
    if t == 0. { // in set
        // return Color::from_rgba(201, 30, 119, 255);
        // return interpolate_colour(BLACK, BLUE, 0.35);
        return BLACK;
    }
    interpolate_colour(BLACK, colour, t as f32)
}

fn complex_to_screen(dimensions: ScreenDimensions, pixel_step: f64, center: (f64, f64), c: Complex) -> Option<(usize, usize)> {
    let dx = c.real - center.0;
    let dy = c.im - center.1;

    let x_move = dx / pixel_step;
    let y_move = dy / pixel_step;

    let x = ((dimensions.x as f64 / 2.) + x_move).floor();
    let y = ((dimensions.y as f64 / 2.) - y_move).floor();

    if x >= dimensions.x as f64 || y >= dimensions.y as f64 || x < 0. || y < 0. {
        return None;
    }

    Some((x as usize, y as usize))
}

// TODO: arbitrary precision when deep zoom
enum ComplexType {
    Double,
    Big
} 

struct RenderInfo {
    dimensions: ScreenDimensions,
    start_y: usize,
    thread_height: usize,
    center: (f64, f64),
    pixel_step: f64,
    max_iterations: u32,
    image: Arc<Mutex<Image>>,
    layers: Layers
}

fn render_image(info: RenderInfo) {
    for x in 0..info.dimensions.x {
        for y in info.start_y..info.start_y+info.thread_height {
            let z = Complex::new(
                (info.center.0 - info.dimensions.x as f64/2.0 * info.pixel_step) + x as f64 * info.pixel_step, 
                (info.center.1 - info.dimensions.y as f64/2.0 * info.pixel_step) + y as f64 * info.pixel_step,
            );
            // let colour: Color = colour_pixel(&info, z);
            let colour: Color = info.layers.colour_pixel(z, info.max_iterations);
        
            let mut im = info.image.lock().unwrap();
            im.set_pixel(x as u32, y as u32, colour);
        }
    }
}

pub struct Visualiser {
    view_dimensions: ScreenDimensions,
    screenshot_dimensions: ScreenDimensions,
    current_dimensions: ScreenDimensions,
    center: (f64, f64),
    pixel_step: f64,
    max_iterations: f32,
    thread_pool: ThreadPool,
    layers: Layers,
    image: Arc<Mutex<Image>>,
    texture: Texture2D, 
    move_speed: f64,
}
impl Visualiser {
    pub fn new(
        pixel_step: f64, 
        max_iterations: f32, 
        view_dimensions: (usize, usize),
        screenshot_dimensions: (usize, usize),
        layers: Layers
    ) -> Visualiser {
        Visualiser { pixel_step, max_iterations, layers,
            view_dimensions: ScreenDimensions::from_tuple(view_dimensions), 
            screenshot_dimensions: ScreenDimensions::from_tuple(screenshot_dimensions),
            current_dimensions: ScreenDimensions::from_tuple(view_dimensions),
            center: (-0.5, 0.0),
            image: Arc::new(Mutex::new(
                Image::gen_image_color(view_dimensions.0 as u16, view_dimensions.1 as u16, 
                                       Color::new(0.0, 0.0, 0.0, 1.0)
            ))),
            texture: Texture2D::empty(),
            move_speed: 1f64, 
            thread_pool: ThreadPool::new(THREADS)
        }
    }

    pub fn load(&mut self, pixel_step: f64, center_x: f64, center_y: f64, max_iterations: f32) {
        self.pixel_step = pixel_step;
        self.center = (center_x, center_y);
        self.max_iterations = max_iterations;
        self.move_speed *= pixel_step / 0.005;
    }

    /// generates and stores the mandlebrot image
    /// for the current parameters
    pub fn generate_image(&mut self) {
        self.layers.generate_palletes(self.max_iterations);
        
        let thread_height = self.current_dimensions.y / THREADS;
        for t in 0..THREADS {
            let render_info = RenderInfo {
                dimensions: self.current_dimensions.clone(),
                start_y: t * thread_height,
                thread_height,
                center: self.center.clone(),
                pixel_step: self.pixel_step.clone(),
                max_iterations: self.max_iterations.clone() as u32,
                image: Arc::clone(&self.image),
                layers: self.layers.clone()
            };
            self.thread_pool.execute(move || {
                render_image(render_info)
            });
        }
        self.thread_pool.join();

        self.texture = Texture2D::from_image(&self.image.clone().lock().unwrap());
    }

    /// draw a generated image to the screen
    pub fn draw(&self) {
        draw_texture(self.texture, 0.0, 0.0, WHITE);
        
        let mut l = 0.0;
        for layer in self.layers.layers.iter() {
            match layer.layer_type {
                LayerType::Colour | LayerType::ColourOrbitTrap(_) => {},
                _ => {continue}
            }
            let size = self.current_dimensions.x as f32 / layer.pallete.len() as f32;
            for (i, colour) in layer.pallete.iter().enumerate() {
                draw_rectangle(
                    i as f32 * size, 
                    l * 10., 
                    size, 
                    10., 
                    *colour
                );
            }
            l += 1.0;
        }

        draw_text(
            &get_fps().to_string(),
            10., 
            10., 
            20., 
            BLACK
        );
        draw_text(
            &self.max_iterations.to_string(),
            self.current_dimensions.x as f32 - 50., 
            10., 
            20., 
            BLACK
        );
    }

    /// lets the user move the view around
    /// 
    /// returns whether the view was moved or not
    fn user_move_view(&mut self, dt: f64) -> bool {
        let (mut moved_y, mut moved_x) = (true, true);

        self.center.1 += self.move_speed * dt * match (is_key_down(KeyCode::W), is_key_down(KeyCode::S)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => {moved_y = false; 0.0}
        };

        self.center.0 += self.move_speed * dt * match (is_key_down(KeyCode::A), is_key_down(KeyCode::D)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => {moved_x = false; 0.0}
        };

        moved_y || moved_x
    }

    /// lets the user zoom in 
    /// 
    /// return whether the user has zoomed or not
    fn user_zoom(&mut self, dt: f64) -> bool {
        let mut zoomed = true;
        
        match (is_key_down(KeyCode::Up), is_key_down(KeyCode::Down)) {
            (true, false) => {
                self.pixel_step *= 1.0 - ZOOM_PERCENT_INC * dt;
                self.move_speed *= 1.0 - ZOOM_PERCENT_INC * dt;
            }
            (false, true) => {
                self.pixel_step *= 1.0 + ZOOM_PERCENT_INC * dt;
                self.move_speed *= 1.0 + ZOOM_PERCENT_INC * dt;
            }
            _ => {zoomed = false}
        }

        zoomed
    }

    fn get_max_iteration_index_higher(&self) -> usize {
        let mut higher = 0;
        for (i, max_iteration) in MAX_ITERATION_STEPS.iter().enumerate() {
            if *max_iteration > self.max_iterations {
                higher = i;
                break;
            }
            if i == MAX_ITERATION_STEPS.len()-1 {
                higher = MAX_ITERATION_STEPS.len();
            }
        } 
        higher
    }

    /// lets the user increase/decrease the max iterations
    /// 
    /// returns whether the max iterations has changed or not
    fn user_change_max_iteration(&mut self, dt: f64) -> bool {
        let mut iter = true;
        
        self.max_iterations += MAX_ITER_INC_SPEED * dt as f32 * match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => {iter = false; 0.0}
        };

        self.max_iterations = match (is_key_pressed(KeyCode::Minus), is_key_pressed(KeyCode::Equal)) {
            (true, false) => {
                iter = true;
                let higher = self.get_max_iteration_index_higher();
                if higher <= 1 {self.max_iterations}
                else {MAX_ITERATION_STEPS[higher-2]}
            }
            (false, true) => {
                iter = true;
                let higher = self.get_max_iteration_index_higher();
                if higher == MAX_ITERATION_STEPS.len() {self.max_iterations}
                else {MAX_ITERATION_STEPS[higher]}
            }
            _ => self.max_iterations
        };

        iter
    }

    /// lets the user shift the pallete length
    /// 
    /// returns if the pallete has been changed or not
    fn user_shift_pallete(&mut self, dt: f64) -> bool {
        let mut pallete = true;

        let add = PALLETE_LENGTH_INC_SPEED * dt as f32 * match (is_key_down(KeyCode::I), is_key_down(KeyCode::P)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => {pallete = false; 0.0}
        };

        if !pallete {return false}

        for layer in self.layers.layers.iter_mut() {
            match layer.layer_type {
                LayerType::Colour | LayerType::ColourOrbitTrap(_) => {
                    layer.change_pallete_length(add, self.max_iterations);
                },
                _ => {continue}
            }
        }

        true
    }

    /// lets the user teleport back to the top of the set
    /// 
    /// returns if a teleport has happened
    fn user_teleport(&mut self) -> bool {
        if !is_key_pressed(KeyCode::T) {return false}

        self.pixel_step = 0.005;
        self.move_speed = 1.;

        true
    }

    /// lets the user export the current view to png
    fn user_export(&mut self) {
        if !is_key_pressed(KeyCode::Key0) {return}

        let mut images_path = std::env::current_dir().unwrap();
        images_path.push("images");

        match fs::create_dir(&images_path) {
            _ => ()
        }

        let datetime = chrono::offset::Local::now();
        let fmt = StrftimeItems::new("%Y%m%d_%H_%M_%S");
        let image_name = format!["{}_{}", datetime.format_with_items(fmt.clone()), self.screenshot_dimensions.as_string()];
        images_path.push(image_name.clone());
        // fs::create_dir(&images_path).unwrap();

        // change dimensions and pixel step for higher quality
        let old_pixel_step = self.pixel_step.clone();
        let screen_height = self.view_dimensions.y as f64 * self.pixel_step;
        self.pixel_step = screen_height / self.screenshot_dimensions.y as f64;
        self.current_dimensions = self.screenshot_dimensions.clone();
        self.image = Arc::new(Mutex::new(
            Image::gen_image_color(self.current_dimensions.x as u16, self.current_dimensions.y as u16, 
                                   Color::new(0.0, 0.0, 0.0, 1.0)
        )));

        self.generate_image();
        let path = &format!["images/{}.png",
            image_name.clone()
        ];
        self.image.clone().lock().unwrap().export_png(path);

        self.pixel_step = old_pixel_step;
        self.current_dimensions = self.view_dimensions.clone();
        self.image = Arc::new(Mutex::new(
            Image::gen_image_color(self.current_dimensions.x as u16, self.current_dimensions.y as u16, 
                                   Color::new(0.0, 0.0, 0.0, 1.0)
        )));

        self.generate_image();
    }

    /// lets the user change the view
    pub fn user_move(&mut self) {
        let dt = get_frame_time() as f64;

        let moved_view = self.user_move_view(dt);
        let zoomed = self.user_zoom(dt);
        let iter = self.user_change_max_iteration(dt);
        let pallete = self.user_shift_pallete(dt);
        let tp = self.user_teleport();

        if is_key_pressed(KeyCode::Z) {
            println!("{} {} = {}x zoom\n{:?}", 
                self.max_iterations, self.pixel_step, 0.005/self.pixel_step, self.center);
        }
        self.user_export();   
    
        if moved_view || zoomed || iter || pallete || tp {
            self.generate_image();
        }
    }

    /// automatically zooms into the centre
    /// speed = percentage increase in zoom per second
    pub fn play(&mut self, speed: f64) {
        let dt = get_frame_time() as f64;
        self.pixel_step *= 1.0 - speed * dt;
        self.generate_image();

        self.draw();
    }
}

pub struct Buhddabrot {
    center: (f64, f64),
    pixel_step: f64,
    max_iterations: f32,
    points: u32,
    coloured: bool,
    texture: Texture2D
}
#[allow(dead_code)]
impl Buhddabrot {
    pub fn new(pixel_step: f64 , max_iterations: f32, points: u32, coloured: bool) -> Buhddabrot {
        Buhddabrot { pixel_step, max_iterations, points, coloured,
            center: (-0.5, 0.0),
            texture: Texture2D::empty()
        }
    }

    pub fn load(&mut self, pixel_step: f64, center_x: f64, center_y: f64) {
        self.pixel_step = pixel_step;
        self.center = (center_x, center_y);
    }

    /// creates an image for a channel
    pub fn generate_channel(&mut self, max_iterations: u32, colour: Color) -> Image {
        let mut image = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, Color::new(0.0, 0.0, 0.0, 1.0));

        let screen = Arc::new(Mutex::new(vec![0u32; WIDTH*HEIGHT]));

        let thread_points = self.points as usize / THREADS;
        let mut threads = Vec::with_capacity(THREADS);
        for _ in 0..THREADS {
            let pixel_step = self.pixel_step.clone();
            let center = self.center.clone();
            let screen = Arc::clone(&screen);
            threads.push(thread::spawn(move || {
                for _ in 0..thread_points {
                    let c = Complex::new(
                        ::rand::thread_rng().gen_range(-2.0..2.0),
                        ::rand::thread_rng().gen_range(-2.0..2.0)
                    );
                    if diverges(c, max_iterations as u32) == 0. {continue}

                    let mut z = c.clone();
                    let mut s = screen.lock().unwrap();
                    for _ in 0..max_iterations as u32 {
                        if let Some(p) = 
                            complex_to_screen(ScreenDimensions::new(WIDTH, HEIGHT), pixel_step, center, z) {
                            s[WIDTH*p.1 + p.0] += 1;
                        }
                        z = z.square() + c;
                        if z.abs_squared() > BAILOUT + 2.0 { break }
                    }
                }
            }));
        }
        for thread in threads {
            thread.join().unwrap();
        }

        let mut max_num = 0;
        for num in screen.lock().unwrap().iter() { 
            if *num > max_num {
                max_num = *num;
            }
        }

        for (i, num) in screen.lock().unwrap().iter().enumerate() {
            let mut c = colour;
            c.a = 1.5 * *num as f32 / max_num as f32;
            image.set_pixel((i / WIDTH) as u32, (i % WIDTH) as u32, c);
        }

        return image
    }

    /// generates and stores the buhddabrot image
    /// for the current hyperparameters
    pub fn generate_image(&mut self) {
        if self.coloured {
            let red = self.generate_channel(
                5000,
                Color::new(1., 0., 0., 1.)
            );
            let green = self.generate_channel(
                500,
                Color::new(0., 1., 0., 1.)
            );
            let blue = self.generate_channel(
                50,
                Color::new(0., 0., 1., 1.)
            );
            let mut main_img = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, Color::new(0.0, 0.0, 0.0, 1.0));
            for i in 0..WIDTH {
                for j in 0..HEIGHT {
                    let c = Color::new(
                        red.get_pixel(i as u32, j as u32).a,
                        green.get_pixel(i as u32, j as u32).a,
                        blue.get_pixel(i as u32, j as u32).a,
                        1.
                    );
                    main_img.set_pixel(i as u32, j as u32, c);
                }
            }
            self.texture = Texture2D::from_image(&main_img);
        } else {
            self.texture = Texture2D::from_image(&self.generate_channel(
                self.max_iterations as u32,
                Color::new(1., 1., 1., 1.)
            ));
        }
        
    }

    /// draw a generated image to the screen
    pub fn draw(&self) {
        draw_texture(self.texture, 0.0, 0.0, WHITE);

        // draw_text(
        //     &get_fps().to_string(), 
        //     10., 
        //     10., 
        //     20., 
        //     WHITE
        // );
    }

    /// lets the user move the view
    pub fn user_move(&mut self) {
        let mut iter = true;

        self.max_iterations += MAX_ITER_INC_SPEED as f32 * match (is_key_pressed(KeyCode::Left), is_key_pressed(KeyCode::Right)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => {iter = false; 0.0}
        };

        if iter {
            self.generate_image();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converges() {
        let a = Complex::new(0f64, 0f64);

        assert_eq!(0.0, diverges(a, 50));
    }

    #[test]
    fn c_diveges() {
        let a = Complex::new(-1.0, 1.0);

        assert!(diverges(a, 50) > 0.0);
    }

    #[test]
    fn complex_screen() {
        let c = Complex::new(-0.5, 0.0);
        let screen = Some((WIDTH / 2, HEIGHT / 2));

        assert_eq!(screen, complex_to_screen(ScreenDimensions::new(WIDTH, HEIGHT), 0.005, (c.real, c.im), c));
    }
}