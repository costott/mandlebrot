// © 2023 costott. All rights reserved. 
// This code is provided for viewing purposes only. Copying, reproduction, 
// or distribution of this code, in whole or in part, in any form or by any 
// means, is strictly prohibited without prior written permission from the 
// copyright owner.

use macroquad::prelude::*;
use chrono::format::strftime::StrftimeItems;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Instant;
use std::f64::consts::PI;
use std::fs;
use ::rand::Rng;
use threadpool::ThreadPool;
use dashu_float::FBig;

pub mod complex;
use complex::*;
pub mod palettes;
pub mod layers;
use layers::Layers;
mod menu;
use menu::Menu;

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
pub const BAILOUT_ORBIT_TRAP: f64 = 20.0;

// user changing view
pub const ZOOM_PERCENT_INC: f64 = 0.5f64;
pub const START_ZOOM_SPEED: f64 = 1f64;
pub const MAX_ITER_INC_SPEED: f32 = 10f32;

pub const THREADS: usize = 12; //12 14 15 17

pub const MIN_FPS: usize = 10;
/// extra fps that must be exceeded before the pixel size decreases
pub const FPS_DROP_EXCESS: usize = (MIN_FPS as f32 * 0.75) as usize;
pub const MAX_QUALITY: usize = 20;
/// if the percentage completed is above this value just after
/// the thread pool has started, the pool will be joined
pub const JOIN_PROGRESS_PERCENT: f32 = 0.05;

pub const MAX_ITERATION_STEPS: [f32; 6] = [250.0, 500.0, 1000.0, 2500.0, 5000.0, 10000.0];

pub struct App {
    visualiser: Visualiser,
    menu: Menu
}
impl App {
    pub async fn new(visualiser: Visualiser) -> App {
        App { visualiser, menu: Menu::new().await }
    }

    pub async fn run(&mut self) {
        let now = Instant::now();
        self.visualiser.generate_image();
        println!("Took {} seconds to generate",  now.elapsed().as_secs_f32());
        self.visualiser.user_move();

        loop {
            self.visualiser.draw();
            if !self.menu.get_editing() {
                self.visualiser.update();
                get_char_pressed(); // flush input queue
            } 

            self.menu.update(&mut self.visualiser).await;

            next_frame().await;
        }
    }
}

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
    use std::f64::consts::PI;

    use crate::BAILOUT_ORBIT_TRAP;

    use super::{
        complex::*,
        menu::DropDownType
    };

    #[derive(Clone, Copy, PartialEq)]
    pub enum OrbitTrapAnalysis {
        Distance,
        Real,
        Imaginary,
        Angle
    }
    impl DropDownType<OrbitTrapAnalysis> for OrbitTrapAnalysis {
        fn get_variants() -> Vec<OrbitTrapAnalysis> {
            vec![
                OrbitTrapAnalysis::Distance,
                OrbitTrapAnalysis::Real,
                OrbitTrapAnalysis::Imaginary,
                OrbitTrapAnalysis::Angle
            ]
        }

        fn get_string(&self) -> String {
            String::from(match self {
                OrbitTrapAnalysis::Distance => "Distance",
                OrbitTrapAnalysis::Real => "Real",
                OrbitTrapAnalysis::Imaginary => "Imaginary",
                OrbitTrapAnalysis::Angle => "Angle"
            })
        }
    }

    #[derive(Clone, PartialEq)]
    pub struct OrbitTrapPoint {
        point: Complex,
        big_point: BigComplex,
        analysis: OrbitTrapAnalysis
    }
    impl OrbitTrapPoint {
        pub fn new(point: (f64, f64), analysis: OrbitTrapAnalysis) -> OrbitTrapPoint {
            OrbitTrapPoint { 
                point: Complex::new(point.0, point.1), 
                big_point: BigComplex::from_f64s(point.0, point.1),
                analysis 
            }
        }

        pub fn default() -> OrbitTrapPoint {
            OrbitTrapPoint::new((0., 0.), OrbitTrapAnalysis::Distance)
        }

        /// returns a vector of the given (double) complex number to the point
        fn vector_double(&self, z: Complex) -> Complex {
            z - self.point
        }
        /// returns a vector of the given (arbitrary) complex number to the point
        fn vector_big(&self, z: &BigComplex) -> BigComplex {
            z - &self.big_point
        }

        /// returns the distance squared between the 
        /// given (double) complex number and the point trap
        pub fn distance2_double(&self, z: Complex) -> f64 {
            // match self.analysis {
            //     OrbitTrapAnalysis::Distance => (z-self.point).abs_squared(),
            //     OrbitTrapAnalysis::Real => (z.real-self.point.real).abs(),
            //     OrbitTrapAnalysis::Imaginary => (z.im-self.point.im).powi(2)
            // }
            (z-self.point).abs_squared()
        }
        /// returns the distance squared between the 
        /// given (arbitrary) complex number and the point trap
        pub fn distance2_big(&self, z: &BigComplex) -> f64 {
            (z-&self.big_point).abs_squared()
        }

        /// returns the maximum possible distance
        /// a complex number can be from the trap
        pub fn greatest_distance2(&self) -> f64 {
            let big_rad = BAILOUT_ORBIT_TRAP.sqrt();
            match self.analysis {
                OrbitTrapAnalysis::Distance => (big_rad + self.point.abs_squared().sqrt()).powi(2),
                OrbitTrapAnalysis::Real => (big_rad + self.point.real.abs()).powi(2),
                OrbitTrapAnalysis::Imaginary => (big_rad + self.point.im.abs()).powi(2),
                OrbitTrapAnalysis::Angle => (2.*PI).powi(2)
            }
            // (big_rad + self.point.abs_squared().sqrt()).powi(2)
        }
    }

    #[derive(Clone, PartialEq)]
    pub struct OrbitTrapCross {
        centre: Complex,
        big_centre: BigComplex,
        pub arm_length: f64,
        analysis: OrbitTrapAnalysis
    }
    impl OrbitTrapCross {
        pub fn new(centre: (f64, f64), arm_length: f64, analysis: OrbitTrapAnalysis) -> OrbitTrapCross {
            OrbitTrapCross { 
                centre: Complex::new(centre.0, centre.1), 
                big_centre: BigComplex::from_f64s(centre.0, centre.1),
                arm_length, analysis
            }
        }

        pub fn default() -> OrbitTrapCross {
            OrbitTrapCross::new((0., 0.), 1., OrbitTrapAnalysis::Distance)
        }

        // TODO: vector

        fn vector_double(&self, _z: Complex) -> Complex {
            Complex::new(0.0, 0.0)
        }
        fn vector_big(&self, _z: &BigComplex) -> BigComplex {
            BigComplex::from_f64s(0.0, 0.0)
        }

        fn distance2(&self, z: Complex) -> f64 {
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
        
        /// returns the shortest distance squared between the 
        /// given (double) complex number and the cross trap
        pub fn distance2_double(&self, z: Complex) -> f64 {
            self.distance2(z)
        }
        /// returns the shortest distance squared between the 
        /// given (arbitrary) complex number and the cross trap
        pub fn distance2_big(&self, z: &BigComplex) -> f64 {
            let z = z.to_complex();
            self.distance2(z)
        }

        /// returns the maximum possible distance
        /// a complex number can be from the trap
        pub fn greatest_distance2(&self) -> f64 {
            (BAILOUT_ORBIT_TRAP.sqrt() + self.centre.abs_squared().sqrt()).powi(2)
        }
    }

    #[derive(Clone, PartialEq)]
    pub struct OrbitTrapCircle {
        centre: Complex,
        big_centre: BigComplex,
        pub radius: f64,
        analysis: OrbitTrapAnalysis
    }
    impl OrbitTrapCircle {
        pub fn new(centre: (f64, f64), radius: f64, analysis: OrbitTrapAnalysis) -> OrbitTrapCircle {
            OrbitTrapCircle { 
                centre: Complex::new(centre.0, centre.1), 
                big_centre: BigComplex::from_f64s(centre.0, centre.1),
                radius, analysis
            }
        }

        pub fn default() -> OrbitTrapCircle {
            OrbitTrapCircle::new((0., 0.), 1., OrbitTrapAnalysis::Distance)
        }

        // TODO: VECTOR

        /// returns the smallest vector of the given (double) complex number to the circle
        fn vector_double(&self, _z: Complex) -> Complex {
            Complex::new(0.0, 0.0)
        }
        /// returns the smallest vector of the given (arbitrary) complex number to the circle
        fn vector_big(&self, _z: &BigComplex) -> BigComplex {
            BigComplex::from_f64s(0.0, 0.0)
        }

        /// returns the shortest distance between the 
        /// given complex number and the circle trap
        pub fn distance2_double(&self, z: Complex) -> f64 {
            ((z-self.centre).abs_squared().sqrt() - self.radius).powi(2)
        }
        /// returns the shortest distance between the 
        /// given complex number and the circle trap
        pub fn distance2_big(&self, z: &BigComplex) -> f64 {
            ((z-&self.big_centre).abs_squared().sqrt() - self.radius).powi(2)
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

    #[derive(Clone)]
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

        pub fn vector_double(&self, z: Complex) -> Complex {
            match self {
                OrbitTrapType::Point(point) => point.vector_double(z),
                OrbitTrapType::Cross(cross) => cross.vector_double(z),
                OrbitTrapType::Circle(circle) => circle.vector_double(z)
            }
        }
        pub fn vector_big(&self, z: &BigComplex) -> BigComplex {
            match self {
                OrbitTrapType::Point(point) => point.vector_big(z),
                OrbitTrapType::Cross(cross) => cross.vector_big(z),
                OrbitTrapType::Circle(circle) => circle.vector_big(z)
            }
        }
        
        /// returns the distance squared between the given complex number and trap
        pub fn distance2_double(&self, z: Complex) -> f64 {
            match self {
                OrbitTrapType::Point(point) => point.distance2_double(z),
                OrbitTrapType::Cross(cross) => cross.distance2_double(z),
                OrbitTrapType::Circle(circle) => circle.distance2_double(z)
            }
        }
        /// returns the distance squared between the given complex number and trap
        pub fn distance2_big(&self, z: &BigComplex) -> f64 {
            match self {
                OrbitTrapType::Point(point) => point.distance2_big(z),
                OrbitTrapType::Cross(cross) => cross.distance2_big(z),
                OrbitTrapType::Circle(circle) => circle.distance2_big(z)
            }
        }

        pub fn get_analysis(&self) -> OrbitTrapAnalysis {
            match self {
                OrbitTrapType::Point(point) => point.analysis,
                OrbitTrapType::Cross(cross) => cross.analysis,
                OrbitTrapType::Circle(circle) => circle.analysis
            }
        }
        pub fn set_analysis(&mut self, new: OrbitTrapAnalysis) {
            match self {
                OrbitTrapType::Point(point) => point.analysis = new,
                OrbitTrapType::Cross(cross) => cross.analysis = new,
                OrbitTrapType::Circle(circle) => circle.analysis = new
            }
        }

        pub fn get_center_re(&self) -> f64 {
            match self {
                OrbitTrapType::Point(point) => point.point.real,
                OrbitTrapType::Cross(cross) => cross.centre.real,
                OrbitTrapType::Circle(circle) => circle.centre.real
            }
        }
        pub fn set_center_re(&mut self, new: f64) {
            match self {
                OrbitTrapType::Point(point) => point.point.real = new,
                OrbitTrapType::Cross(cross) => cross.centre.real = new,
                OrbitTrapType::Circle(circle) => circle.centre.real = new
            }
        }
        pub fn get_center_im(&self) -> f64 {
            match self {
                OrbitTrapType::Point(point) => point.point.im,
                OrbitTrapType::Cross(cross) => cross.centre.im,
                OrbitTrapType::Circle(circle) => circle.centre.im
            }
        }
        pub fn set_center_im(&mut self, new: f64) {
            match self {
                OrbitTrapType::Point(point) => point.point.im = new,
                OrbitTrapType::Cross(cross) => cross.centre.im = new,
                OrbitTrapType::Circle(circle) => circle.centre.im = new
            }
        }
    }
    impl DropDownType<OrbitTrapType> for OrbitTrapType {
        fn get_variants() -> Vec<OrbitTrapType> {
            vec![
                OrbitTrapType::Point(OrbitTrapPoint::default()),
                OrbitTrapType::Circle(OrbitTrapCircle::default()),
                OrbitTrapType::Cross(OrbitTrapCross::default())
            ]
        }

        fn get_string(&self) -> String {
            String::from(match self {
                OrbitTrapType::Point(_) => "Point",
                OrbitTrapType::Circle(_) => "Circle",
                OrbitTrapType::Cross(_) => "Cross"
            })
        }
    }
    impl PartialEq for OrbitTrapType {
        fn eq(&self, other: &Self) -> bool {
            match self {
                OrbitTrapType::Point(_) => match other {
                    OrbitTrapType::Point(_) => true,
                    _ => false
                },
                OrbitTrapType::Cross(_) => match other {
                    OrbitTrapType::Cross(_) => true,
                    _ => false
                },
                OrbitTrapType::Circle(_) => match other {
                    OrbitTrapType::Circle(_) => true,
                    _ => false
                }
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

    pub fn screen_size() -> ScreenDimensions {
        ScreenDimensions {
            x: screen_width() as usize,
            y: screen_height() as usize
        }
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

    fn numpixels(&self) -> usize {
        self.x * self.y
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
        lerp(c1.a, c2.a, fraction)
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

struct ThreadSplitter {
    x_excess: usize,
    y_excess: usize,
    x_end: usize, 
    y_end: usize
}

struct Renderer {
    dimensions: ScreenDimensions,
    start_y: usize,
    thread_height: usize,
    center: ComplexType,
    pixel_step: f64,
    max_iterations: u32,
    image: Arc<Mutex<Image>>,
    layers: Layers,
    quality: usize,
    thread_cancel: Arc<AtomicBool>,
    reference_orbit: Arc<Option<ReferenceOrbit>>,
    progress_tracker: Arc<Mutex<usize>>
}
impl Renderer {
    fn render_image(self) { 
        let split = ThreadSplitter {
            x_excess: self.dimensions.x % self.quality,
            y_excess: self.thread_height % self.quality,
            x_end: self.dimensions.x / self.quality,
            y_end: self.thread_height / self.quality
        };

        match self.center {
            ComplexType::Double(_) => self.render_double(&split),
            // ComplexType::Big(_) => self.render_arbitrary(&split)
            ComplexType::Big(_) => self.render_arbitrary_perturbed(&split)
        }
    }

    fn render_double(&self, split: &ThreadSplitter) {
        for x in 0..=split.x_end {
            for y in 0..=split.y_end {
                let z = ComplexType::Double(Complex::new(
                    (self.center.real_f64() - self.dimensions.x as f64/2.0 * self.pixel_step) + x as f64 * self.pixel_step * self.quality as f64, 
                    (self.center.im_f64() - self.dimensions.y as f64/2.0 * self.pixel_step) + (self.start_y+y*self.quality) as f64 * self.pixel_step,
                ));
                self.set_pixels(z, x, y, split);

                if self.thread_cancel.load(Ordering::Relaxed) {
                    return;
                }
            }
        }
    }

    fn render_arbitrary(&self, split: &ThreadSplitter) {
        let center = match self.center {
            ComplexType::Big(ref c) => c.clone(),
            ComplexType::Double(_) => panic!("center needs to be made arbitrary for arbitrary precision")
        };
        let dims = BigComplex::from_f64s(self.dimensions.x as f64, self.dimensions.y as f64);
        // not sure what precision this should be yet - might need to change dynamically
        let pixel_step = FBig::try_from(self.pixel_step).unwrap().with_precision(100).value();

        let quality = FBig::try_from(self.quality).unwrap().with_precision(100).value();

        let mut x_add = FBig::ZERO;
        let start_y = FBig::try_from(self.start_y).unwrap() * &pixel_step;
        let mut y_add;

        let topleft = center - (dims/2.0) * &pixel_step;

        for x in 0..=split.x_end {
            y_add = FBig::ZERO;
            for y in 0..=split.y_end {
                let z = ComplexType::Big(BigComplex::new(
                    &topleft.real + &x_add * &pixel_step, 
                    &topleft.im + &start_y + &y_add * &pixel_step,
                ));
                y_add += &quality;
                self.set_pixels(z, x, y, split);

                if self.thread_cancel.load(Ordering::Relaxed) {
                    return;
                }
            }
            x_add += &quality;
        }
    }

    fn render_arbitrary_perturbed(&self, split: &ThreadSplitter) {
        let reference_orbit = match self.reference_orbit.as_ref() {
            Some(orbit) => orbit,
            None => panic!("perturbation needs a reference orbit")
        };

        let half_width = (self.dimensions.x as f64 / 2.) * &self.pixel_step;
        let half_height = (self.dimensions.y as f64 / 2.) * &self.pixel_step;

        let start_y = self.start_y as f64 * &self.pixel_step;
        
        for y in 0..=split.y_end {
            for x in 0..=split.x_end {
                let dc = Complex::new(
                    -&half_width + (x * &self.quality) as f64 * &self.pixel_step,
                    -&half_height + &start_y + (y * &self.quality) as f64 * &self.pixel_step
                );
                self.set_pixels_perturbation(dc, &reference_orbit.ref_z, reference_orbit.max_ref_iteration, x, y, split);

                if self.thread_cancel.load(Ordering::Relaxed) {
                    return;
                }
            }
        }
    }

    fn set_pixels(&self, z: ComplexType, x: usize, y: usize, split: &ThreadSplitter) {
        let colour: Color = self.layers.colour_pixel(z, self.max_iterations);
            
        let mut im = self.image.lock().unwrap();

        let width = if x == split.x_end {split.x_excess} else {self.quality};
        let height = if y == split.y_end {split.y_excess} else {self.quality};
        
        for i in 0..width {
            for j in 0..height {
                im.set_pixel(
                    (x*self.quality+i) as u32, 
                    (self.start_y + y*self.quality + j) as u32, 
                    colour
                );
            }
        }

        *self.progress_tracker.lock().unwrap() += width*height;
    }

    fn set_pixels_perturbation(&self, dc: Complex, ref_z: &Vec<Complex>, max_ref_iteration: usize, x: usize, y: usize, split: &ThreadSplitter) {
        let colour: Color = self.layers.colour_pixel_implementors_perturbed(dc, ref_z, max_ref_iteration, self.max_iterations);
            
        let mut im = self.image.lock().unwrap();

        let width = if x == split.x_end {split.x_excess} else {self.quality};
        let height = if y == split.y_end {split.y_excess} else {self.quality};
        
        for i in 0..width {
            for j in 0..height {
                im.set_pixel(
                    (x*self.quality+i) as u32, 
                    (self.start_y + y*self.quality + j) as u32, 
                    colour
                );
            }
        }

        *self.progress_tracker.lock().unwrap() += width*height;
    }
}

struct ReferenceOrbit {
    /// the reference orbit, starting from 0 + 0i
    ref_z: Vec<Complex>,
    /// the iteration just before the referencre orbit diverged
    max_ref_iteration: usize
}
impl ReferenceOrbit {
    fn new(center: &BigComplex, max_iterations: usize) -> ReferenceOrbit {
        let mut ref_z: Vec<Complex> = Vec::with_capacity(max_iterations);
        let mut max_ref_iteration= 0;

        let mut z = BigComplex::from_f64s(0., 0.);
        for i in 0..max_iterations {
            ref_z.push(z.to_complex());
            if z.abs_squared() < BAILOUT_ORBIT_TRAP {
                z = z.square() + center;
                max_ref_iteration = i;
            } else {
                break;
            }
        }
    
        ReferenceOrbit { ref_z, max_ref_iteration }
    }
}

/// responsible for holding the data required whilst
/// generating the image to be saved to the machine
struct Exporter {
    exporting: bool,
    images_path: std::path::PathBuf,
    name: String,
    dims: ScreenDimensions,
    image: Arc<Mutex<Image>>,
    progress_tracker: Arc<Mutex<usize>>
}
impl Exporter {
    fn new() -> Exporter {
        let mut images_path = std::env::current_dir().unwrap();
        images_path.push("images");

        match fs::create_dir(&images_path) {
            _ => ()
        }

        Exporter { 
            exporting: false, 
            name: String::from(""), 
            dims: ScreenDimensions::from_tuple((0, 0)),
            images_path: images_path,
            image: Arc::new(Mutex::new(Image::empty())),
            progress_tracker: Arc::new(Mutex::new(0))
        }
    }

    fn start_export(
        &mut self, 
        name: &String, 
        dimensions: ScreenDimensions,
        current_dimensions: &ScreenDimensions,
        visualiser_pixel_step: f64
    ) -> f64 {
        self.name = name.clone();
        self.dims = dimensions;

        let mut images_path = std::env::current_dir().unwrap();
        images_path.push("images");

        match fs::create_dir(&images_path) {
            _ => ()
        }

        // preserve height of the image
        let screen_height = current_dimensions.y as f64 * visualiser_pixel_step;
        let pixel_step = screen_height / self.dims.y as f64;

        self.image = Arc::new(Mutex::new(
            Image::gen_image_color(self.dims.x as u16, self.dims.y as u16, 
                                   Color::new(0.0, 0.0, 0.0, 1.0)
        )));
        self.progress_tracker = Arc::new(Mutex::new(0));
        self.exporting = true;

        pixel_step
    }

    fn cancel_export(&mut self) {
        self.image = Arc::new(Mutex::new(Image::empty()));
        self.exporting = false;
    }

    fn update(&mut self, thread_pool: &ThreadPool) {
        if !(thread_pool.active_count() == 0 && thread_pool.queued_count() == 0) {
            return
        }

        self.finish_export();
    }

    fn finish_export(&mut self) {
        let datetime = chrono::offset::Local::now();
        let date = StrftimeItems::new("%Y%m%d");
        let time = StrftimeItems::new("%H_%M_%S");
        self.name = self.name.replace("[date]", &format!["{}", datetime.format_with_items(date.clone())]);
        self.name = self.name.replace("[time]", &format!["{}", datetime.format_with_items(time.clone())]);
        self.name = format!["{}_{}", self.name, self.dims.as_string()];
        self.images_path.push(self.name.clone());

        let path = &format!["images/{}.png",
            self.name.clone()
        ];
        self.image.lock().unwrap().export_png(path);

        self.exporting = false;
    }
}

pub struct Visualiser {
    current_dimensions: ScreenDimensions,
    center: ComplexType,
    pixel_step: f64,
    max_iterations: f32,
    thread_pool: ThreadPool,
    rendering: bool,
    /// used to cancel all currently running threads
    thread_cancel: Arc<AtomicBool>,
    layers: Layers,
    image: Arc<Mutex<Image>>,
    texture: Texture2D, 
    /// the percentage increase in zoom per second
    move_speed: f64,
    /// how many pixels each complex number generated represents
    pub quality: usize,
    /// quality before decreasing quality when user stopped moving
    saved_quality: usize,
    arb_precision: bool,
    moving: bool,
    exporter: Exporter,
    progress_tracker: Arc<Mutex<usize>>,
    render_start_time: Instant,
    last_render_time: f32
}
impl Visualiser {
    pub fn new(
        pixel_step: f64, 
        max_iterations: f32, 
        view_dimensions: (usize, usize),
        layers: Layers
    ) -> Visualiser {
        Visualiser { pixel_step, max_iterations, layers,
            current_dimensions: ScreenDimensions::from_tuple(view_dimensions),
            center: ComplexType::Double(Complex::new(-0.5, 0.0)),
            image: Arc::new(Mutex::new(
                Image::gen_image_color(view_dimensions.0 as u16, view_dimensions.1 as u16, 
                                       Color::new(0.0, 0.0, 0.0, 1.0)
            ))),
            texture: Texture2D::empty(),
            move_speed: START_ZOOM_SPEED, 
            thread_pool: ThreadPool::new(5),
            rendering: false,
            thread_cancel: Arc::new(AtomicBool::new(false)),
            quality: 2,
            saved_quality: 1,
            arb_precision: false,
            moving: false,
            exporter: Exporter::new(),
            progress_tracker: Arc::new(Mutex::new(0)),
            render_start_time: Instant::now(),
            last_render_time: f32::INFINITY
        }
    }

    pub fn load(&mut self, pixel_step: f64, center_x: f64, center_y: f64, max_iterations: f32) {
        self.pixel_step = pixel_step;
        self.center = ComplexType::Double(Complex::new(center_x, center_y));
        self.max_iterations = max_iterations;
        self.move_speed *= pixel_step / 0.005;
    }

    pub fn load_big(&mut self, pixel_step: f64, center_x: &str, center_y: &str, max_iterations: f32) {
        self.pixel_step = pixel_step;
        self.center = ComplexType::Big(BigComplex::from_string_base10(center_x, center_y));
        self.update_precision();
        self.max_iterations = max_iterations;
        self.move_speed *= pixel_step / 0.005;
    }

    pub fn set_view_dimensions(&mut self, dimensions: &ScreenDimensions) {
        self.cancel_current_render();

        self.current_dimensions = dimensions.clone();
        self.image = Arc::new(Mutex::new(
            Image::gen_image_color(self.current_dimensions.x as u16, self.current_dimensions.y as u16, 
                                   Color::new(0.0, 0.0, 0.0, 1.0)
        )));
        
        self.generate_image();
    }

    fn cancel_current_render(&mut self) {
        self.thread_cancel.store(true, Ordering::Relaxed);
        self.thread_pool.join();
        self.thread_cancel.store(false, Ordering::Relaxed);
        self.rendering = false;
    }  

    fn generate_image(&mut self) {
        if self.rendering {
            if self.moving { return }
            self.quality += 1;
            self.cancel_current_render();
        }
        self.rendering = true;

        self.layers.generate_palettes(self.max_iterations);

        self.progress_tracker = Arc::new(Mutex::new(0));
        self.generate_given_image(self.image.clone(), self.current_dimensions.clone(), self.pixel_step, Arc::clone(&self.progress_tracker));
        self.render_start_time = Instant::now();
        
        Texture2D::delete(&self.texture);
        self.texture = Texture2D::from_image(&self.image.lock().unwrap());
    }

    /// generates and stores the mandlebrot image
    /// for the current parameters
    pub fn generate_given_image(
        &mut self, 
        image: Arc<Mutex<Image>>, 
        dimensions: ScreenDimensions, 
        pixel_step: f64,
        progress_tracker: Arc<Mutex<usize>>
    ) {
        let center = match &self.center {
            ComplexType::Big(c) => if !self.arb_precision {
                ComplexType::Double(c.to_complex())
            } else {
                self.center.clone()
            },
            ComplexType::Double(_) => self.center.clone()
        };

        let reference_orbit = if !self.arb_precision {
            Arc::new(None)
        } else {
            Arc::new(Some(ReferenceOrbit::new(
                match &self.center {
                    ComplexType::Double(_) => panic!("arbitrary precision needs an arbitrary precision center"),
                    ComplexType::Big(c) => c
                }, 
                self.max_iterations as usize
            )))
        };
        
        let thread_height = dimensions.y / THREADS;

        for t in 0..THREADS {
            let renderer = Renderer {
                dimensions: dimensions.clone(),
                start_y: t * thread_height,
                thread_height,
                center: center.clone(),
                pixel_step: pixel_step.clone(),
                max_iterations: self.max_iterations.clone() as u32,
                image: Arc::clone(&image),
                layers: self.layers.clone(),
                quality: self.quality.clone(),
                thread_cancel: Arc::clone(&self.thread_cancel),
                reference_orbit: Arc::clone(&reference_orbit),
                progress_tracker: Arc::clone(&progress_tracker)
            };
            self.thread_pool.execute(move || {
                renderer.render_image()
            });
        }
    }

    /// draw a generated image to the screen
    pub fn draw(&mut self) {
        let start_x = screen_width() - self.current_dimensions.x as f32;
        draw_texture(self.texture, start_x, 0.0, WHITE);

        draw_text(
            &get_fps().to_string(),
            start_x + 30., 
            10., 
            20., 
            BLACK
        );
        draw_text(
            &self.quality.to_string(),
            start_x + 60.,
            10.,
            20.,
            BLACK
        );
        draw_text(
            &self.max_iterations.to_string(),
            screen_width() - 50., 
            10., 
            20., 
            BLACK
        );
    }

    fn update(&mut self) {
        if self.exporter.exporting {
            self.exporter.update(&self.thread_pool);
            return;
        }
        self.user_move();

        self.manage_render_time();
    }

    fn move_view_double(&self, center: &Complex, dt: f64) -> (ComplexType, bool) {
        let (mut moved_y, mut moved_x) = (true, true);

        let mut center = center.clone();

        center.real += self.move_speed * dt * match (is_key_down(KeyCode::A), is_key_down(KeyCode::D)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => {moved_y = false; 0.0}
        };

        center.im += self.move_speed * dt * match (is_key_down(KeyCode::W), is_key_down(KeyCode::S)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => {moved_x = false; 0.0}
        };

        (ComplexType::Double(center), moved_x || moved_y)
    }

    fn move_view_big(&self, center: &BigComplex, dt: f64) -> (ComplexType, bool) {
        let (mut moved_y, mut moved_x) = (true, true);

        let mut center = center.clone();

        let movement = FBig::try_from(self.move_speed * dt).unwrap().with_precision(0).value();

        center.real += movement.clone() * FBig::try_from(match (is_key_down(KeyCode::A), is_key_down(KeyCode::D)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => {moved_x = false; 0.0}
        }).unwrap();

        center.im += movement * FBig::try_from(match (is_key_down(KeyCode::W), is_key_down(KeyCode::S)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => {moved_y = false; 0.0}
        }).unwrap();

        (ComplexType::Big(center), moved_x || moved_y)
    }

    /// lets the user move the view around
    /// 
    /// returns whether the view was moved or not
    fn user_move_view(&mut self, dt: f64) -> bool {
        let moved;

        (self.center, moved) = match self.center {
            ComplexType::Double(c) => {
                self.move_view_double(&c, dt)
            },
            ComplexType::Big(ref c) => {
                self.move_view_big(c, dt)
            }
        };

        moved
    }

    /// lets the user zoom in 
    /// 
    /// return whether the user has zoomed or not
    fn user_zoom(&mut self, _dt: f64) -> bool {
        let mut zoomed = true;
        
        match (is_key_down(KeyCode::Up), is_key_down(KeyCode::Down)) {
            (true, false) => {
                self.pixel_step *= 1.0 - ZOOM_PERCENT_INC / (get_fps() as f64 + ZOOM_PERCENT_INC + 1.0);
                self.move_speed *= 1.0 - ZOOM_PERCENT_INC / (get_fps() as f64 + ZOOM_PERCENT_INC + 1.0);
            }
            (false, true) => {
                self.pixel_step *= 1.0 + ZOOM_PERCENT_INC / (get_fps() as f64 + ZOOM_PERCENT_INC + 1.0);
                self.move_speed *= 1.0 + ZOOM_PERCENT_INC / (get_fps() as f64 + ZOOM_PERCENT_INC + 1.0);
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
        self.max_iterations = self.max_iterations.max(1.0);

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

    /// lets the user teleport back to the top of the set
    /// 
    /// returns if a teleport has happened
    fn user_teleport(&mut self) -> bool {
        if !is_key_pressed(KeyCode::T) {return false}

        self.pixel_step = 0.005;
        self.move_speed = 1.;

        true
    }

    fn start_export(&mut self, name: &String, dimensions: ScreenDimensions) {
        let pixel_step = self.exporter.start_export(name, dimensions,  &self.current_dimensions, self.pixel_step);
        self.quality = 1;
        self.generate_given_image(
            Arc::clone(&self.exporter.image), 
            self.exporter.dims.clone(), 
            pixel_step,
            Arc::clone(&self.exporter.progress_tracker)
        );
    }

    fn finish_render(&mut self) {
        self.rendering = false;
        self.last_render_time = self.render_start_time.elapsed().as_secs_f32();
        if self.last_render_time <= 1. / (MIN_FPS + FPS_DROP_EXCESS) as f32 && self.quality > 1 {
            self.quality -= 1;
        }
    }

    /// manage render parameters while an image is being rendered
    fn manage_render_time(&mut self) {
        if !self.rendering { return }

        Texture2D::delete(&self.texture);
        self.texture = Texture2D::from_image(&self.image.lock().unwrap());

        if self.quality < self.saved_quality && self.moving {
            self.quality = self.saved_quality;
        }

        if self.last_render_time <= 1. / MIN_FPS as f32 {
            self.thread_pool.join();
            Texture2D::delete(&self.texture);
            self.texture = Texture2D::from_image(&self.image.lock().unwrap());

            self.finish_render();
            return;
        }

        if self.thread_pool.active_count() == 0 && self.thread_pool.queued_count() == 0 {
            self.finish_render();
            return;
        }

        if self.render_start_time.elapsed().as_secs_f32() < 1. / MIN_FPS as f32 { return }

        if self.moving {
            self.quality += 1;
            self.cancel_current_render();
        }
    }

    /// lets the user change the view
    pub fn user_move(&mut self) {
        self.update_precision();

        let dt = get_frame_time() as f64;

        let moved_view = self.user_move_view(dt);
        let zoomed = self.user_zoom(dt);
        let iter = self.user_change_max_iteration(dt);
        let tp = self.user_teleport();

        if is_key_pressed(KeyCode::Z) {
            println!("{} {} = {}x zoom\n{:?}\nreal: {} im: {}", 
                self.max_iterations, self.pixel_step, 0.005/self.pixel_step, self.center, self.center.real_string(), self.center.im_string());
        }
    
        if moved_view || zoomed || iter || tp {
            self.moving = true;
            self.generate_image();
        // user stopped moving - increase quality
        } else if !self.rendering && self.quality > 1 {
            self.moving = false;
            self.quality /= 2;
            self.generate_image();
        } else {
            self.moving = false;
        }
    }

    fn update_precision(&mut self) {
        if self.pixel_step <= 2e-16 && !self.arb_precision {
            self.arb_precision = true;
            self.center = self.center.make_big();
            self.quality += 1 ;
        } else if self.pixel_step > 2e-16 && self.arb_precision {
            self.arb_precision = false;
        }
    }

    fn set_pixel_step(&mut self, new: f64) {
        if new <= 0.0 { return }
        self.pixel_step = new;
        self.move_speed = new * (START_ZOOM_SPEED / 0.005);
        self.update_precision();
    }

    /// automatically zooms into the centre
    /// `speed` = percentage increase in zoom per second
    /// `target_pixel_step` = stops zooming once it reaches a certain zoom
    pub fn play(&mut self, speed: f64, target_pixel_step: Option<f64>) {
        match target_pixel_step {
            Some(t) => {
                if self.pixel_step <= t {
                    self.moving = false;
                    if self.quality > 1 {
                        self.quality = 1;
                        self.generate_image();
                    }
                    self.draw();
                    return;
                } else {
                    self.pixel_step *= 1.0 - speed / (get_fps() as f64 + speed + 1.0);
                    self.moving = true;
                }
            },
            None => {
                self.pixel_step *= 1.0 - speed / (get_fps() as f64 + speed + 1.0);
                self.moving = true;
            }
        }

        self.manage_render_time();
        self.update_precision();
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
            Texture2D::delete(&self.texture);
            self.texture = Texture2D::from_image(&main_img);
        } else {
            Texture2D::delete(&self.texture);
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