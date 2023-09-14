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
use num_cpus;

pub mod complex;
use complex::*;
pub mod palettes;
pub mod layers;
use layers::{Layer, Layers};
mod menu;
use menu::Menu;

// width+height only used for the buhddabrot
pub const WIDTH: usize = 600;
pub const HEIGHT: usize = 600;

// both are techically the actual value squared
pub const BAILOUT: f64 = 4f64;
pub const EPSILON: f64 = 1e-6f64;

/// pixel step where anything smaller will use arbitrary precision
const ARB_PRECISION_THRESHOLD: f64 = 2e-16;

// 3D 
pub const H2: f64 = 1.5;
pub const ANGLE: f64 = -45.;

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

/// the minimum distance between percentages for a new timestamp 
/// to be able to be added
const MIN_TIMESTAMP_DIFF: f32 = 0.1;

fn get_str_between<'a>(text: &'a str, start_pattern: &str, end_pattern: &str) -> &'a str {
    let start = text.find(start_pattern).and_then(|i| Some(i+1)).unwrap_or(0);
    let end = text[start..].find(end_pattern).unwrap_or(text.len()-start) + start;
    &text[start..end]
}

fn calculate_checksum(text: &str) -> usize {
    let oneline = text.split_ascii_whitespace().collect::<Vec<&str>>().join("");
    let chars: Vec<&str> = oneline.split("").collect();
    let mut total = 0;
    for c in chars {
        if let Ok(num) = c.parse::<usize>() {
            total += num
        }
    }
    
    total
}

fn check_chesum(text: &str) -> bool {
    let checksum = calculate_checksum(&text);
    let lines: Vec<&str> = text.split("\n").collect();

    // check if checksums match
    match lines.last() {
        None => false,
        Some(real_sum) => {
            let extra_sum = calculate_checksum(real_sum);
            match real_sum.parse::<usize>() {
                Err(_) => false,
                Ok(real_sum) => {
                 checksum - extra_sum == real_sum
                }
            }
        }
    }

}

fn get_unique_image_name(name: &String, dims: &ScreenDimensions, folder: &std::path::Path) -> String {
    for path in fs::read_dir(&folder).expect("unable to read folder") {
        if let Ok(path) = path {
            if &path.file_name().into_string().unwrap() == &format!["{}_{}.png", name, dims.to_string()] {
                return get_unique_image_name(&format!["{}(1)", name], dims, folder);
            }
        }
    }
    name.to_owned()
}

fn get_unique_video_name(name: &String, folder: &std::path::Path) -> String {
    for path in fs::read_dir(&folder).expect("unable to read folder") {
        if let Ok(path) = path {
            if &path.file_name().into_string().unwrap() == name {
                return get_unique_video_name(&format!["{}(1)", name], folder);
            }
        }
    }
    name.to_owned()
}

pub struct App {
    visualiser: Visualiser,
    menu: Menu,
    running: bool
}
impl App {
    pub async fn new(visualiser: Visualiser) -> App {
        let menu = Menu::new(&visualiser).await;
        App { visualiser, menu, running: true }
    }

    pub async fn run(&mut self) {
        let now = Instant::now();
        self.visualiser.generate_image();
        println!("Took {} seconds to generate",  now.elapsed().as_secs_f32());
        self.visualiser.user_move();

        while self.running {
            self.visualiser.draw();
            if !self.menu.get_editing() {
                self.visualiser.update();
                get_char_pressed(); // flush input queue
            } 

            self.menu.update(&mut self.visualiser).await;
            self.menu.leave_menu.update(&mut self.running, &mut self.visualiser);

            next_frame().await;
        }
    }
}

#[derive(Clone)]
pub struct JuliaSeed {
    double: Complex,
    big: BigComplex
}
impl JuliaSeed {
    fn new(real: f64, im: f64) -> JuliaSeed {
        JuliaSeed { 
            double: Complex::new(real, im), 
            big: BigComplex::from_f64s(real, im)
        }
    }

    fn set_real(&mut self, real: f64) {
        self.double.real = real;
        self.big = BigComplex::from_complex(self.double.clone());
    }

    fn set_im(&mut self, im: f64) {
        self.double.im = im;
        self.big = BigComplex::from_complex(self.double.clone());
    }

    fn lerp_seeds(seed1: &JuliaSeed, seed2: &JuliaSeed, percent: f64) -> JuliaSeed {
        JuliaSeed::new(
            lerpf64(seed1.double.real, seed2.double.real, percent),
            lerpf64(seed1.double.im, seed2.double.im, percent)
        )
    }

    fn get_export_string(&self) -> String {
        format!["{},{}", self.double.real.to_string(), self.double.im.to_string()]
    }

    fn import_from_str(seed: &str) -> JuliaSeed {
        let values = seed.split(",").collect::<Vec<&str>>();
        JuliaSeed::new(
            values[0].parse::<f64>().unwrap(),
            values[1].parse::<f64>().unwrap()
        )
    }
}

#[derive(Clone)]
pub enum Fractal {
    Mandelbrot,
    Julia(JuliaSeed)
}
impl Fractal {
    pub fn iterate_double(&self, z: &mut Complex, c: Complex) {
        match self {
            Fractal::Mandelbrot => {*z = z.square() + c},
            Fractal::Julia(seed) => {*z = z.square() + seed.double}
        }
    }

    pub fn iterate_big(&self, z: &mut BigComplex, c: &BigComplex) {
        match self {
            Fractal::Mandelbrot => {*z = &z.square() + c},
            Fractal::Julia(seed) => {*z = &z.square() + &seed.big}
        }
    }

    pub fn is_mandelbrot(&self) -> bool {
        match self {
            Fractal::Mandelbrot => true,
            _ => false
        }
    }

    pub fn is_julia(&self) -> bool {
        match self {
            Fractal::Julia(_) => true,
            _ => false
        }
    }

    pub fn unwrap_julia_seed(&mut self) -> &mut JuliaSeed {
        match self {
            Fractal::Julia(seed) => seed,
            _ => panic!("not julia")
        }
    }

    fn lerp_fractal(fractal1: &Fractal, fractal2: &Fractal, percent: f64) -> Fractal {
        match fractal2 {
            Fractal::Mandelbrot => Fractal::Mandelbrot,
            Fractal::Julia(seed2) => match fractal1 {
                Fractal::Mandelbrot => Fractal::Julia(seed2.clone()),
                Fractal::Julia(seed1) => Fractal::Julia(JuliaSeed::lerp_seeds(seed1, seed2, percent))
            }
        }
    }

    fn get_export_string(&self) -> String {
        match self {
            Fractal::Mandelbrot => String::from("Mandelbrot"),
            Fractal::Julia(seed) => format!["Julia({})", seed.get_export_string()]
        }
    }

    fn import_from_str(fractal: &str) -> Fractal {
        if fractal == "Mandelbrot" {
            return Fractal::Mandelbrot;
        }
        if &fractal[0..=4] == "Julia" {
            return Fractal::Julia(JuliaSeed::import_from_str(get_str_between(fractal, "(", ")")));
        }

        panic!("unknown fractal")
    }
}

pub mod orbit_trap {
    use std::f64::consts::PI;

    use crate::{get_str_between, lerpf64};

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
    impl OrbitTrapAnalysis {
        fn export_num(&self) -> &str {
            match self {
                OrbitTrapAnalysis::Distance => "0",
                OrbitTrapAnalysis::Real => "1",
                OrbitTrapAnalysis::Imaginary => "2",
                OrbitTrapAnalysis::Angle => "3"
            }
        }

        fn import_from_num(num: char) -> OrbitTrapAnalysis {
            match num {
                '0' => OrbitTrapAnalysis::Distance,
                '1' => OrbitTrapAnalysis::Real,
                '2' => OrbitTrapAnalysis::Imaginary,
                '3' => OrbitTrapAnalysis::Angle,
                c => panic!("no orbit trap analysis for {c}")
            }
        }
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
        pub fn greatest_distance2(&self, bailout2: f64) -> f64 {
            let big_rad = bailout2.sqrt();
            match self.analysis {
                OrbitTrapAnalysis::Distance => (big_rad + self.point.abs_squared().sqrt()).powi(2),
                OrbitTrapAnalysis::Real => (big_rad + self.point.real.abs()).powi(2),
                OrbitTrapAnalysis::Imaginary => (big_rad + self.point.im.abs()).powi(2),
                OrbitTrapAnalysis::Angle => (2.*PI).powi(2)
            }
            // (big_rad + self.point.abs_squared().sqrt()).powi(2)
        }

        fn interpolate_points(p1: &OrbitTrapPoint, p2: &OrbitTrapPoint, percent: f64) -> OrbitTrapPoint {
            let centre = ComplexType::lerp_complex(
                &ComplexType::Double(p1.point),&ComplexType::Double(p2.point), percent, false, 1.
            );

            OrbitTrapPoint::new(
                (centre.real_f64(), centre.im_f64()),
                p1.analysis
            )
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

        fn vector_double(&self, z: Complex) -> Complex {
            let vector;
            let x_dist = z.real-self.centre.real;
            let y_dist = z.im-self.centre.im;
            if self.centre.im - self.arm_length <= z.im && z.im <= self.centre.im + self.arm_length &&
                self.centre.real - self.arm_length <= z.real && z.real <= self.centre.real + self.arm_length {
                vector = if x_dist.abs() <= y_dist.abs() {
                    Complex::new(x_dist, 0.)
                } else { 
                    Complex::new(0., y_dist)
                }
            } else if x_dist.abs() < y_dist.abs() {
                vector = z-
                    Complex::new(self.centre.real, self.centre.im+y_dist.signum()*self.arm_length)
            } else {
                vector = z-
                    Complex::new(self.centre.real+x_dist.signum()*self.arm_length, self.centre.im)
            }

            vector
        }
        fn vector_big(&self, _z: &BigComplex) -> BigComplex {
            BigComplex::from_f64s(0.0, 0.0)
        }

        fn distance2(&self, z: Complex) -> f64 {
            self.vector_double(z).abs_squared()
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
        pub fn greatest_distance2(&self, bailout2: f64) -> f64 {
            // (bailout2.sqrt() + self.centre.abs_squared().sqrt()).powi(2)
            match self.analysis {
                OrbitTrapAnalysis::Distance => (bailout2.sqrt() + self.centre.abs_squared().sqrt()).powi(2),
                OrbitTrapAnalysis::Real => (bailout2.sqrt() + self.centre.real.abs() - self.arm_length).powi(2),
                OrbitTrapAnalysis::Imaginary => (bailout2.sqrt() + self.centre.im.abs() - self.arm_length).powi(2),
                OrbitTrapAnalysis::Angle => (2.*PI).powi(2)
            }
        }

        fn interpolate_crosses(p1: &OrbitTrapCross, p2: &OrbitTrapCross, percent: f64) -> OrbitTrapCross {
            let centre = ComplexType::lerp_complex(
                &ComplexType::Double(p1.centre),&ComplexType::Double(p2.centre), percent, false, 1.
            );

            OrbitTrapCross::new(
                (centre.real_f64(), centre.im_f64()),
                lerpf64(p1.arm_length, p2.arm_length, percent),
                p1.analysis
            )
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

        /// returns the smallest vector of the given (double) complex number to the circle
        fn vector_double(&self, z: Complex) -> Complex {
            // Complex::new(0.0, 0.0)
            let dist_rem = self.distance2_double(z).sqrt() % self.radius;
            ((z - self.centre) / (z-self.centre).abs_squared().sqrt()) * dist_rem
        }
        /// returns the smallest vector of the given (arbitrary) complex number to the circle
        fn vector_big(&self, z: &BigComplex) -> BigComplex {
            let dist_rem = self.distance2_big(z).sqrt() % self.radius;
            ((z - &self.big_centre) / (z-&self.big_centre).abs_squared().sqrt()) * dist_rem
        }

        /// returns the shortest distance squared between the 
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
        pub fn greatest_distance2(&self, bailout2: f64) -> f64 {
            let big_rad = bailout2.sqrt();
            
            match self.analysis {
                OrbitTrapAnalysis::Distance => f64::max(
                        big_rad - (self.radius - self.centre.abs_squared().sqrt()), 
                        self.radius
                    ).powi(2),
                OrbitTrapAnalysis::Real => big_rad + self.centre.real - self.radius,
                OrbitTrapAnalysis::Imaginary => big_rad + self.centre.im - self.radius,
                OrbitTrapAnalysis::Angle => (2.*PI).powi(2)
            }
        }

        /// minimum possible distance a point can be from the trap
        pub fn minimum_distance2(&self, bailout2: f64) -> f64 {
            // smallest distance between radius of trap and bailout
            let circle_distance = (self.radius - self.centre.abs_squared().sqrt()) - bailout2.sqrt();
            f64::max(0.0, circle_distance).powi(2)
        }

        fn interpolate_circles(p1: &OrbitTrapCircle, p2: &OrbitTrapCircle, percent: f64) -> OrbitTrapCircle {
            let centre = ComplexType::lerp_complex(
                &ComplexType::Double(p1.centre),&ComplexType::Double(p2.centre), percent, false, 1.
            );

            OrbitTrapCircle::new(
                (centre.real_f64(), centre.im_f64()),
                lerpf64(p1.radius, p2.radius, percent),
                p1.analysis
            )
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
        pub fn greatest_distance2(&self, bailout2: f64) -> f64 {
            match self {
                OrbitTrapType::Point(point) => point.greatest_distance2(bailout2),
                OrbitTrapType::Cross(cross) => cross.greatest_distance2(bailout2),
                OrbitTrapType::Circle(circle) => circle.greatest_distance2(bailout2)
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

        pub fn interpolate_similar_traps(trap1: &OrbitTrapType, trap2: &OrbitTrapType, percent: f64) -> OrbitTrapType {
            match trap1 {
                OrbitTrapType::Point(point) => {
                    let other_point = match trap2 {
                        OrbitTrapType::Point(p) => p,
                        _ => panic!("traps aren't similar")
                    };
                    OrbitTrapType::Point(OrbitTrapPoint::interpolate_points(point, other_point, percent))
                },
                OrbitTrapType::Cross(cross) => {
                    let other_cross = match trap2 {
                        OrbitTrapType::Cross(c) => c,
                        _ => panic!("traps aren't similar")
                    };
                    OrbitTrapType::Cross(OrbitTrapCross::interpolate_crosses(cross, other_cross, percent))
                },
                OrbitTrapType::Circle(circle) => {
                    let other_circle = match trap2 {
                        OrbitTrapType::Circle(c) => c,
                        _ => panic!("traps aren't similar")
                    };
                    OrbitTrapType::Circle(OrbitTrapCircle::interpolate_circles(circle, other_circle, percent))
                }
            }
        }

        fn export_num(&self) -> &str {
            match self {
                OrbitTrapType::Point(_) => "0",
                OrbitTrapType::Cross(_) => "1",
                OrbitTrapType::Circle(_) => "2"
            }
        }

        fn export_extra_param(&self) -> String {
            match self {
                OrbitTrapType::Point(_) => String::from(""),
                OrbitTrapType::Cross(cross) => cross.arm_length.to_string(),
                OrbitTrapType::Circle(circle) => circle.radius.to_string()
            }
        }

        pub fn get_export_str(&self) -> String {
            format!["{}({},{}){}[{}]", 
            self.export_num(), 
            self.get_center_re().to_string(), self.get_center_im().to_string(),
            self.get_analysis().export_num(),
            self.export_extra_param()]
        }

        pub fn import_from_str(trap: &str) -> OrbitTrapType {
            let trap_type = trap.chars().nth(0).unwrap();
            
            let centre: Vec<&str> = get_str_between(trap, "(", ")").split(",").collect();
            let centre_re = centre[0].parse::<f64>().unwrap();
            let centre_im = centre[1].parse::<f64>().unwrap();
            let centre = (centre_re, centre_im);

            let analysis_n = trap.find(")").unwrap() + 1;
            let analysis = OrbitTrapAnalysis::import_from_num(trap.chars().nth(analysis_n).unwrap());
    
            let specific_param = get_str_between(trap, "[", "]").parse::<f64>().unwrap_or(0.0);

            match trap_type {
                '0' => OrbitTrapType::Point(OrbitTrapPoint::new(centre, analysis)),
                '1' => OrbitTrapType::Cross(OrbitTrapCross::new(centre, specific_param, analysis)),
                '2' => OrbitTrapType::Circle(OrbitTrapCircle::new(centre, specific_param, analysis)),
                c => panic!("no orbit trap type for {c}")
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
    fn to_string(&self) -> String {
        let res = format!["{}x{}", self.x, self.y];
        String::from( match (self.x, self.y) {
            (1920, 1080) => "1080p",
            (3840, 2160) => "4k",
            (7680, 4320) => "8k",
            (_, _) => &res
        })
    }

    fn from_str(dims: &str) -> Result<ScreenDimensions, &'static str> {
        match dims {
            "1080p" => Ok(ScreenDimensions::from_tuple(ScreenDimensions::tuple_1080p())),
            "4k" => Ok(ScreenDimensions::from_tuple(ScreenDimensions::tuple_4k())),
            "8k" => Ok(ScreenDimensions::from_tuple(ScreenDimensions::tuple_8k())),
            other =>  {
                let dim_tuple = other.split("x").collect::<Vec<&str>>();
                if dim_tuple.len() != 2 { return Err("incorrect number of dimensions") }
                let x = dim_tuple[0].parse::<usize>();
                let x = match x {
                    Err(_) => return Err("unable to parse width"),
                    Ok(width) => width
                };
                let y = dim_tuple[1].parse::<usize>();
                let y = match y {
                    Err(_) => return Err("unable to parse height"),
                    Ok(height) => height
                };
                Ok(ScreenDimensions::from_tuple((x, y)))
            }
        }
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

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1f32 - t) * a + t * b
}

fn lerpf64(a: f64, b: f64, t: f64) -> f64 {
    let output = (1. - t) * a + t * b;
    match output == (1. - t) * a || output == t * b {
        false => output,
        true => lerp_fbig(
            FBig::try_from(a).unwrap(), FBig::try_from(b).unwrap(), &FBig::try_from(t).unwrap()
        ).to_f64().value()
    }
}

fn lerpf64_ln(a: f64, b: f64, t: f64) -> f64 {
    lerpf64(a.ln(), b.ln(), t).exp()
}

// fn lerpf64_mexp(a: f64, b: f64, t: f64, m: f64) -> f64 {
//     match a <= b {
//         true => lerpf64((m*a).exp(), (m*b).exp(), t).ln() / m,
//         false => lerpf64((-m*a).exp(), (-m*b).exp(), t).ln() / -m
//     }
// }

fn lerpf64_pow(a: f64, b: f64, t: f64, p: f64) -> f64 {
    lerpf64(a, b, t.powf(p))
}

fn lerp_fbig(a: FBig, b: FBig, t: &FBig) -> FBig {
    (FBig::ONE - t) * a + t * b
}

#[allow(unused)] // will be used once pixel step can become arbitrary
fn lerp_fbig_ln(a: FBig, b: FBig, t: &FBig) -> FBig {
    FBig::exp(&lerp_fbig(a.ln(), b.ln(), &t))
}

// fn lerp_fbig_mexp(a: FBig, b: FBig, t: &FBig, m: &FBig) -> FBig {
//     lerp_fbig((m * &a).exp(), (m * &b).exp(), t).ln() / m
// }

fn lerp_fbig_pow(a: FBig, b: FBig, t: &FBig, p: &FBig) -> FBig {
    lerp_fbig(a, b, &t.powf(p))
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
    fractal: Fractal,
    dimensions: ScreenDimensions,
    start_y: usize,
    thread_height: usize,
    center: ComplexType,
    pixel_step: f64,
    max_iterations: u32,
    bailout2: f64,
    image: Arc<Mutex<Image>>,
    layers: Layers,
    quality: usize,
    thread_cancel: Arc<AtomicBool>,
    can_cancel: bool,
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
            ComplexType::Big(_) => match self.fractal {
                Fractal::Mandelbrot => self.render_arbitrary_perturbed(&split),
                _ => self.render_arbitrary(&split)
            }
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

                if self.thread_cancel.load(Ordering::Relaxed) && self.can_cancel {
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

                if self.thread_cancel.load(Ordering::Relaxed) && self.can_cancel {
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

                if self.thread_cancel.load(Ordering::Relaxed) && self.can_cancel {
                    return;
                }
            }
        }
    }

    fn set_pixels(&self, z: ComplexType, x: usize, y: usize, split: &ThreadSplitter) {
        let colour: Color = self.layers.colour_pixel(&self.fractal, z, self.max_iterations,  self.bailout2);
            
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
        let colour: Color = self.layers.colour_pixel_implementors_perturbed(
            &self.fractal, dc, ref_z, max_ref_iteration, self.max_iterations, self.bailout2
        );
            
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
    fn new(fractal: &Fractal, center: &BigComplex, max_iterations: usize, bailout2: f64) -> ReferenceOrbit {
        let mut ref_z: Vec<Complex> = Vec::with_capacity(max_iterations);
        let mut max_ref_iteration= 0;

        let mut z = BigComplex::from_f64s(0., 0.);
        for i in 0..max_iterations {
            ref_z.push(z.to_complex());
            if z.abs_squared() < bailout2 {
                fractal.iterate_big(&mut z, center);
                max_ref_iteration = i;
            } else {
                break;
            }
        }
    
        ReferenceOrbit { ref_z, max_ref_iteration }
    }
}

/// stores the elements of the visualiser that need to be saved
struct VisualiserParams {
    fractal: Fractal,
    center_re: String,
    center_im: String,
    magnification: String,
    max_iterations: String,
    bailout2: String,
    layers: Layers
}
impl VisualiserParams {
    fn empty() -> VisualiserParams {
        VisualiserParams { 
            fractal: Fractal::Mandelbrot,
            center_re: String::from(""), 
            center_im: String::from(""), 
            magnification: String::from(""), 
            max_iterations: String::from(""), 
            bailout2: String::from(""), 
            layers: Layers::new(vec![Layer::default()], true)
        }
    }

    fn get_params(visualiser: &Visualiser) -> VisualiserParams {
        VisualiserParams { 
            fractal: visualiser.fractal.clone(),
            center_re: visualiser.center.real_string(), 
            center_im: visualiser.center.im_string(), 
            magnification: visualiser.get_magnification().to_string(), 
            max_iterations: (visualiser.max_iterations as usize).to_string(), 
            bailout2: visualiser.bailout2.to_string(), 
            layers: visualiser.layers.clone()
        }
    }

    fn format_params(&self) -> String {
        let contents = format!("\
{}
center (re): {}
center (im): {}
magnification: {}
max iterations: {}
bailout2: {}

{}", 
        self.fractal.get_export_string(), 
        self.center_re, self.center_im, self.magnification, self.max_iterations, self.bailout2,
        self.layers.get_export_string());

        contents
    }

    fn save_params(&self, path: &std::path::PathBuf) {
        let mut contents = self.format_params();
        contents.push_str(&calculate_checksum(&contents).to_string());

        fs::write(path, contents).expect("unable to save visualiser")
    }

    fn import_from_str(params: &str) -> VisualiserParams {
        let mut lines = params.split("\n").collect::<Vec<&str>>();

        let fractal = lines.remove(0);

        let main_params: &Vec<String> = &lines[0..=4].iter().map(|l| l.split(": ").last().unwrap().to_string()).collect();

        VisualiserParams {
            fractal: Fractal::import_from_str(fractal),
            center_re: main_params[0].clone(),
            center_im: main_params[1].clone(),
            magnification: main_params[2].clone(),
            max_iterations: main_params[3].clone(),
            bailout2: main_params[4].clone(),
            layers: Layers::import_from_file(&lines[6..lines.len()])
        }
    }

    fn get_timestamp_save_string(timestamp: &VideoTimestamp) -> String {
        let params = VisualiserParams {
            fractal: timestamp.fractal.clone(),
            center_re: timestamp.center.real_string(),
            center_im: timestamp.center.im_string(),
            magnification: (0.005 / timestamp.pixel_step).to_string(),
            max_iterations: (timestamp.max_iterations as usize).to_string(),
            bailout2: timestamp.bailout2.to_string(),
            layers: timestamp.layers.clone()
        };

        params.format_params()
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
    progress_tracker: Arc<Mutex<usize>>,
    visualiser_params: VisualiserParams
}
impl Exporter {
    fn new() -> Exporter {
        let mut images_path = std::env::current_dir().unwrap();
        images_path.push("images");

        match fs::create_dir(&images_path) {
            _ => {}
        }
        Exporter::add_examples(&images_path);

        Exporter { 
            exporting: false, 
            name: String::from(""), 
            dims: ScreenDimensions::from_tuple((0, 0)),
            images_path: images_path,
            image: Arc::new(Mutex::new(Image::empty())),
            progress_tracker: Arc::new(Mutex::new(0)),
            visualiser_params: VisualiserParams::empty()
        }
    }

    fn add_examples(images_path: &std::path::PathBuf) {
        let mut example_path = images_path.clone();
        example_path.push("examples");

        match fs::create_dir(&example_path) {
            _ => {}
        }

        let examples = vec![
            include_bytes!("../assets/examples/images/example1.txt").to_vec(),
            include_bytes!("../assets/examples/images/example2.txt").to_vec(),
            include_bytes!("../assets/examples/images/example3.txt").to_vec(),
            include_bytes!("../assets/examples/images/example4.txt").to_vec(),
            include_bytes!("../assets/examples/images/example5.txt").to_vec(),
            include_bytes!("../assets/examples/images/example6.txt").to_vec(),
        ];

        for (i, example) in examples.iter().enumerate() {
            let mut eg = example_path.clone();
            eg.push(format!["example{}.txt", i+1]);
            fs::write(eg, example).unwrap();
        }
    }

    fn start_export(
        &mut self, 
        name: &String, 
        dimensions: ScreenDimensions,
        current_dimensions: &ScreenDimensions,
        visualiser_pixel_step: f64,
        visualiser_params: VisualiserParams
    ) -> f64 {
        self.name = name.clone();
        self.dims = dimensions;
        self.visualiser_params = visualiser_params;

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

        let name = get_unique_image_name(&self.name, &self.dims, &self.images_path);

        let image_name = format!["{}_{}", name, self.dims.to_string()];
        let path = &format!["images/{}.png",
            image_name.clone()
        ];
        self.image.lock().unwrap().export_png(path);

        let mut save_path = self.images_path.clone();
        save_path.push(format!["{}-save.txt", name]);
        self.visualiser_params.save_params(&save_path);

        self.exporting = false;
    }
}

#[derive(Clone)]
pub struct VideoTimestamp {
    fractal: Fractal,
    center: ComplexType,
    pixel_step: f64,
    max_iterations: f32,
    bailout2: f64,
    layers: Layers,
    percent: f32
}
impl VideoTimestamp {
    fn new(visualiser: &Visualiser, percent: f32) -> VideoTimestamp {
        VideoTimestamp { 
            fractal: visualiser.fractal.clone(),
            center: visualiser.center.clone(), 
            pixel_step: visualiser.pixel_step, 
            max_iterations: visualiser.max_iterations,
            bailout2: visualiser.bailout2, 
            layers: visualiser.layers.clone(), 
            percent
        }
    }

    fn from_params(params: VisualiserParams, percent: f32) -> VideoTimestamp {
        let mut center = ComplexType::Double(Complex::new(0. ,0.));
        center.update_real_from_string(params.center_re);
        center.update_im_from_string(params.center_im);

        VideoTimestamp { 
            fractal: params.fractal,
            center, 
            pixel_step: 0.005 / params.magnification.parse::<f64>().unwrap(), 
            max_iterations: params.max_iterations.parse::<f32>().unwrap(), 
            bailout2: params.bailout2.parse::<f64>().unwrap(), 
            layers: params.layers, 
            percent
        }
    }

    fn lerp_timestamps(timestamp1: &VideoTimestamp, timestamp2: &VideoTimestamp, percent: f32) -> VideoTimestamp {
        let percent = ((percent - timestamp1.percent) / (timestamp2.percent - timestamp1.percent)) as f64;

        let pixel_step = lerpf64_ln(timestamp1.pixel_step, timestamp2.pixel_step, percent);

        // TODO: fix interpolating complex
        // let arb_precision = pixel_step <= ARB_PRECISION_THRESHOLD;
        // let z1 = timestamp1.pixel_step;
        // let z2 = timestamp2.pixel_step;
        // let p = f64::min(z1/z2, z2/z1);

        VideoTimestamp { 
            fractal: Fractal::lerp_fractal(&timestamp1.fractal, &timestamp2.fractal, percent),
            // center: ComplexType::lerp_complex(&timestamp1.center, &timestamp2.center, percent, arb_precision, p), 
            center: timestamp2.center.clone(),
            pixel_step, 
            max_iterations: lerp(timestamp1.max_iterations, timestamp2.max_iterations, percent as f32), 
            bailout2: lerpf64(timestamp1.bailout2, timestamp2.bailout2, percent), 
            layers: Layers::lerp_layers(&timestamp1.layers, &timestamp2.layers, percent), 
            percent: percent as f32
        }
    }

    fn get_save(&self) -> String {
        let mut params = VisualiserParams::get_timestamp_save_string(&self);
        params.push_str(&format!["{}\n", self.percent]);
        params
    }

    fn import_from_str(timestamp: &str) -> VideoTimestamp {
        // split into lines to remove percent
        let mut lines = timestamp.split("\n").collect::<Vec<&str>>();
        if lines[0] == "" { lines.remove(0); }
        if lines[lines.len()-1] == "" { lines.pop(); }

        let percent = lines.pop().unwrap().parse::<f32>().unwrap();
        let timestamp = lines.join("\n");

        let params = VisualiserParams::import_from_str(&timestamp);
        VideoTimestamp::from_params(params, percent)
    }
}

#[derive(Clone)]
pub struct VideoRecorder {
    /// holds the timestamps in any order for editing
    timestamps: Vec<VideoTimestamp>,
    /// holds the timestamps in the correct order for recording
    sorted_timestamps: Vec<VideoTimestamp>,
    previewing: bool,
    /// stores whether the recording has changed since last export 
    /// so if it hasn't the export can just be resumed
    changed: bool,
    frames: usize,
    completed_frames: usize,
    exporting: bool,
    needs_resume: bool,
    videos_path: std::path::PathBuf,
    name: String,
    dims: ScreenDimensions,
    /// converts the timestamp's pixel step for the given dimensions
    // pixel_step_multiplier: f64,
    image: Arc<Mutex<Image>>,
    progress_tracker: Arc<Mutex<usize>>
}
impl VideoRecorder {
    fn new() -> VideoRecorder {
        let mut videos_path = std::env::current_dir().unwrap();
        videos_path.push("videos");

        match fs::create_dir(&videos_path) {
            _ => ()
        }
        VideoRecorder::add_examples(&videos_path);

        VideoRecorder { 
            timestamps: Vec::new(), 
            sorted_timestamps: Vec::new(),
            previewing: false,
            needs_resume: false,
            changed: true,
            frames: 0, 
            completed_frames: 0,
            exporting: false, 
            videos_path, 
            name: String::from(""), 
            dims: ScreenDimensions::from_tuple((0, 0)), 
            // pixel_step_multiplier: 0.0,
            image: Arc::new(Mutex::new(Image::empty())),
            progress_tracker: Arc::new(Mutex::new(0))
        }
    }

    fn add_examples(videos_path: &std::path::PathBuf) {
        let mut example_path = videos_path.clone();
        example_path.push("examples");

        match fs::create_dir(&example_path) {
            _ => {}
        }

        let examples = vec![
            include_bytes!("../assets/examples/videos/example1.txt").to_vec(),
            include_bytes!("../assets/examples/videos/example2.txt").to_vec(),
        ];

        for (i, example) in examples.iter().enumerate() {
            let mut eg = example_path.clone();
            eg.push(format!["example{}.txt", i+1]);
            fs::write(eg, example).unwrap();
        }
    }

    fn sort_timestamps(&mut self) {
        self.sorted_timestamps = self.timestamps.clone();
        self.sorted_timestamps.sort_by_key(|t| (t.percent*100.) as u32);

        self.changed = true;
    }
    
    fn get_new_timestamp_percent(&self) -> Option<f32> {
        match self.timestamps.len() {
            0 => {return Some(0.0)},
            1 => {
                // get boundary furtherst away from the other
                let zero = self.timestamps[0].percent - 0.0;
                let one = 1.0 - self.timestamps[0].percent;
                if zero > one {
                    return Some(0.0)
                } else {
                    return Some(1.0)
                }
            },
            _ => {}
        }

        let mut percentages: Vec<f32> = self.sorted_timestamps.clone().iter().map(|ts| ts.percent).collect();
        percentages.insert(0, 0.);
        percentages.push(1.);
        // maximum differences between percentages
        let mut max_percent_diff = 0.0;
        // the percentage to add the new number at
        let mut percent_to_add = 0.0;
        for i in 0..percentages.len()-1 {
            let this_diff = percentages[i+1] - percentages[i];
            let this_add = percentages[i] + this_diff/2.;
            if this_diff < max_percent_diff { continue }

            if this_diff > max_percent_diff {
                max_percent_diff = this_diff;
                percent_to_add = this_add;
            }
            // prioritises percentages closer to the midpoint
            if this_diff == max_percent_diff && (this_add-0.5).abs() < (percent_to_add-0.5).abs() {
                percent_to_add = this_add;
            }
        }

        match max_percent_diff > MIN_TIMESTAMP_DIFF {
            true => Some(percent_to_add),
            false => None
        }
    }

    /// adds the new timestamp after placing it at the correct percent,
    /// returning whether it was successeful or not
    fn new_timestamp(&mut self, new_timestamp: &mut VideoTimestamp) -> bool {
        let percent = self.get_new_timestamp_percent();
        if percent.is_none() {
            return false;
        }

        new_timestamp.percent = percent.unwrap();
        self.timestamps.push(new_timestamp.clone());
        self.sort_timestamps();

       true
    }

    fn change_timestamp_percent(&mut self, unsorted_i: usize, new_percent: f32) {
        self.timestamps[unsorted_i].percent = new_percent;
        self.sort_timestamps();
    }   

    fn delete_timestamp(&mut self, unsorted_i: usize) {
        self.timestamps.remove(unsorted_i);
        self.sort_timestamps();
    }

    fn get_timestamp_at_percent(&self, percent: f32) -> Option<VideoTimestamp> {
        assert!(0.0 <= percent && percent <= 1.0);
        if self.sorted_timestamps.len() < 2 { return None }
        if percent < self.sorted_timestamps[0].percent || 
           percent > self.sorted_timestamps.last().unwrap().percent { return None}
        
        let mut timestamp = None;
        for i in 0..self.sorted_timestamps.len()-1 {
            if self.sorted_timestamps[i+1].percent < percent { continue }

            timestamp = Some(VideoTimestamp::lerp_timestamps(
                &self.sorted_timestamps[i],& self.sorted_timestamps[i+1], percent
            ));
            break;
        }

        timestamp
    }

    fn get_progress(&self) -> f32 {
        let pixels = self.dims.numpixels();
        let this_frame = self.progress_tracker.lock().unwrap().clone();

        (pixels * self.completed_frames + this_frame) as f32 / ( pixels * self.frames ) as f32
    }

    fn save_video(&self, path: &std::path::PathBuf, dimensions: ScreenDimensions) {
        let mut contents = String::from("");

        contents.push_str(&format!["{},{}\n", dimensions.to_string(), self.frames.to_string()]);

        for timestamp in self.sorted_timestamps.iter() {
            contents.push_str(&format!["{}ts\n", timestamp.get_save()]);
        }

        contents.push_str(&calculate_checksum(&contents).to_string());

        fs::write(path, contents).expect("unable to save video")
    }

    fn can_export(&self) -> bool {
        self.sorted_timestamps.len() >= 2 &&
            self.sorted_timestamps[0].percent == 0. &&
            self.sorted_timestamps.last().unwrap().percent == 1.
    }

    fn start_export(
        &mut self, 
        name: &String, 
        dimensions: ScreenDimensions, 
        current_dimensions: &ScreenDimensions,
        time: usize,
        fps: usize
    ) {
        assert!(self.can_export());

        self.frames = time * fps;
        self.completed_frames = 0;
        self.progress_tracker = Arc::new(Mutex::new(0));
        self.dims = dimensions.clone();
        self.changed = false;
        
        self.sort_timestamps();
        // convert pixel steps to required quality
        let pixel_step_multiplier = current_dimensions.y as f64 / self.dims.y as f64;
        for timestamp in self.sorted_timestamps.iter_mut() {
            timestamp.pixel_step *= pixel_step_multiplier;
        }

        self.exporting = true;

        self.videos_path = std::env::current_dir().unwrap();
        self.videos_path.push("videos");

        let datetime = chrono::offset::Local::now();
        let date = StrftimeItems::new("%Y%m%d");
        let time_now = StrftimeItems::new("%H_%M_%S");
        let mut name = name.replace("[date]", &format!["{}", datetime.format_with_items(date.clone())]);
        name = name.replace("[time]", &format!["{}", datetime.format_with_items(time_now.clone())]);

        self.name = get_unique_video_name(&name, &self.videos_path);

        self.videos_path.push(&self.name);

        match fs::create_dir(&self.videos_path) {
            _ => ()
        }

        let mut save_path = self.videos_path.clone();
        save_path.push(format!["save.txt"]);
        self.save_video(&save_path, dimensions);
    }   

    /// resumes the currently loaded video (provided it hasn't changed)
    fn resume_export(&mut self) {
        assert!(!self.changed);
        self.exporting = true;
        self.needs_resume = true;
    }

    /// # Returns
    /// Some(time stamp) if a new frame needs to be rendered
    /// None if no new frame needs to be rendered
    fn update(&mut self, thread_pool: &ThreadPool) -> Option<VideoTimestamp> {
        if !(thread_pool.active_count() == 0 && thread_pool.queued_count() == 0) {
            return None;
        }
        
        if self.completed_frames == self.frames {
            self.finish_frame();
            self.finish_recording();
            return None
        }

        self.finish_frame();
        self.next_frame()
    }

    fn next_frame(&mut self) -> Option<VideoTimestamp> {
        self.image = Arc::new(Mutex::new(
            Image::gen_image_color(self.dims.x as u16, self.dims.y as u16, 
                                   Color::new(0.0, 0.0, 0.0, 1.0)
        )));

        let percent = self.completed_frames as f32 / self.frames as f32;
        self.completed_frames += 1;
        self.needs_resume = false;

        self.get_timestamp_at_percent(percent)
    }

    fn finish_frame(&mut self) {
        if self.completed_frames == 0 || self.needs_resume { return }

        let image_name = format!["{}{}", 
            "0".repeat(self.frames.to_string().chars().count() - self.completed_frames.to_string().chars().count()),
            self.completed_frames      
        ];
        let path = &format!["videos/{}/{}.png",
            self.name,
            image_name.clone()
        ];
        self.image.lock().unwrap().export_png(path);

        self.progress_tracker = Arc::new(Mutex::new(0));
    }

    fn finish_recording(&mut self) {
        // pixel steps changed due to transform, so undo
        self.sort_timestamps();
        self.exporting = false;
    }

    fn cancel_export(&mut self) {
        self.exporting = false;
        self.image = Arc::new(Mutex::new(Image::empty()));
        self.completed_frames -= 1;
    }

    fn import_from_file(&mut self, file_path: &std::path::PathBuf) {
        let save_file = fs::read_to_string(file_path).expect("Unable to read file");

        if !check_chesum(&save_file) { return }

        let mut lines: Vec<&str> = save_file.split("\n").collect();

        let config = lines.remove(0).split(",").collect::<Vec<&str>>();
        if config.len() != 2 { return }
        let dims = match ScreenDimensions::from_str(config[0]) {
            Err(_) => return,
            Ok(d) => d
        };
        let frames = match config[1].parse::<usize>() {
            Err(_) => return,
            Ok(f) => f
        };

        let full = lines.join("\n");
        let mut timestamps = full.split("ts").collect::<Vec<&str>>();
        timestamps.pop(); // remove checksum
        
        self.timestamps = Vec::new();
        for timestamp in timestamps {
            self.timestamps.push(VideoTimestamp::import_from_str(timestamp))
        }
        self.sort_timestamps();

        self.dims = dims;
        self.frames = frames;
        self.changed = false;
        self.needs_resume = true;

        // count rendered frames
        let folder = file_path.as_path().parent().unwrap();
        let vid = fs::read_dir(folder).expect("unable to get video folder");
        self.completed_frames = 0;
        for path in vid {
            if std::path::Path::new(&path.unwrap().path()).extension().unwrap() == "png" {
                self.completed_frames += 1;
            }
        }

        self.name = folder.components().last().unwrap().as_os_str().to_str().unwrap().to_owned();
    }
}

pub struct Visualiser {
    fractal: Fractal,
    current_dimensions: ScreenDimensions,
    center: ComplexType,
    pixel_step: f64,
    max_iterations: f32,
    /// the squared distance of the bailout
    bailout2: f64,
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
    video_recorder: VideoRecorder,
    progress_tracker: Arc<Mutex<usize>>,
    render_start_time: Instant,
    last_render_time: f32
}
impl Visualiser {
    pub fn new(
        fractal: Fractal,
        pixel_step: f64, 
        max_iterations: f32, 
        bailout: f64,
        view_dimensions: (usize, usize),
        layers: Layers
    ) -> Visualiser {
        Visualiser { fractal, pixel_step, max_iterations, layers,
            bailout2: bailout.powi(2),
            current_dimensions: ScreenDimensions::from_tuple(view_dimensions),
            center: ComplexType::Double(Complex::new(-0.5, 0.0)),
            image: Arc::new(Mutex::new(
                Image::gen_image_color(view_dimensions.0 as u16, view_dimensions.1 as u16, 
                                       Color::new(0.0, 0.0, 0.0, 1.0)
            ))),
            texture: Texture2D::empty(),
            move_speed: START_ZOOM_SPEED, 
            thread_pool: ThreadPool::new((num_cpus::get_physical()-1).max(1)),
            rendering: false,
            thread_cancel: Arc::new(AtomicBool::new(false)),
            quality: 2,
            saved_quality: 1,
            arb_precision: false,
            moving: false,
            exporter: Exporter::new(),
            video_recorder: VideoRecorder::new(),
            progress_tracker: Arc::new(Mutex::new(0)),
            render_start_time: Instant::now(),
            last_render_time: f32::INFINITY
        }
    }

    pub fn set_fractal(&mut self, new_fractal: Fractal) {
        self.fractal = new_fractal;
        self.quality = MAX_QUALITY;
        self.set_pixel_step(0.005);
        self.generate_image();
    }

    pub fn get_magnification(&self) -> f64 {
        0.005 / self.pixel_step
    }

    pub fn load(&mut self, pixel_step: f64, center_x: f64, center_y: f64, max_iterations: f32) {
        self.set_pixel_step(pixel_step);
        self.center = ComplexType::Double(Complex::new(center_x, center_y));
        self.max_iterations = max_iterations;
    }

    pub fn load_big(&mut self, pixel_step: f64, center_x: &str, center_y: &str, max_iterations: f32) {
        self.set_pixel_step(pixel_step);
        self.center = ComplexType::Big(BigComplex::from_string_base10(center_x, center_y));
        self.update_precision();
        self.max_iterations = max_iterations;
    }

    pub fn load_timestamp(&mut self, timestamp: &VideoTimestamp) {
        self.fractal = timestamp.fractal.clone();
        self.center = timestamp.center.clone();
        self.set_pixel_step(timestamp.pixel_step);
        self.max_iterations = timestamp.max_iterations;
        self.bailout2 = timestamp.bailout2;
        self.layers = timestamp.layers.clone();

        self.layers.generate_palettes(self.max_iterations);

        self.update_precision();
    }

    fn load_params(&mut self, params: VisualiserParams) {
        // todo: remove all unwrapping so even less chance of invalid data crashing

        self.fractal = params.fractal;
        self.center.update_real_from_string(params.center_re);
        self.center.update_im_from_string(params.center_im);
        self.set_pixel_step(0.005 / params.magnification.parse::<f64>().unwrap());
        self.max_iterations = params.max_iterations.parse::<f32>().unwrap();
        self.bailout2 = params.bailout2.parse::<f64>().unwrap();
        self.layers = params.layers;
    }

    pub fn import_from_file(&mut self, file_path: &std::path::PathBuf) {
        let save_file = fs::read_to_string(file_path).expect("Unable to read file");

        if !check_chesum(&save_file) { return }

        let mut lines: Vec<&str> = save_file.split("\n").collect();
        lines.pop(); // remove checksum

        let params_string = lines.join("\n");
        let params = VisualiserParams::import_from_str(&params_string);
        self.load_params(params);

        self.update_precision();
        self.generate_image();
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
            if self.moving || self.video_recorder.previewing { return }
            self.quality += 1;
            self.cancel_current_render();
        }
        self.rendering = true;

        self.layers.generate_palettes(self.max_iterations);

        self.progress_tracker = Arc::new(Mutex::new(0));
        self.generate_given_image(
            self.image.clone(), self.current_dimensions.clone(), None, self.pixel_step, None, 
            self.quality, Arc::clone(&self.progress_tracker), true
        );
        self.render_start_time = Instant::now();
        
        Texture2D::delete(&self.texture);
        self.texture = Texture2D::from_image(&self.image.lock().unwrap());
    }

    fn get_needed_center(center: ComplexType, pixel_step: f64) -> ComplexType {
        let arb_precision = pixel_step <= ARB_PRECISION_THRESHOLD;

        match &center {
            ComplexType::Big(c) => if !arb_precision {
                ComplexType::Double(c.to_complex())
            } else {
                center.clone()
            },
            ComplexType::Double(_) => center.clone()
        }
    }

    /// generates and stores the mandlebrot image
    /// for the current parameters
    /// 
    /// param is None, the visualiser's is used
    pub fn generate_given_image(
        &mut self, 
        image: Arc<Mutex<Image>>, 
        dimensions: ScreenDimensions, 
        fractal: Option<Fractal>,
        pixel_step: f64,
        center: Option<ComplexType>,
        quality: usize,
        progress_tracker: Arc<Mutex<usize>>,
        can_cancel: bool
    ) {
        let arb_precision = pixel_step <= ARB_PRECISION_THRESHOLD;

        let center = Visualiser::get_needed_center(
            center.unwrap_or(self.center.clone()),
            pixel_step
        );
        let fractal = fractal.unwrap_or(self.fractal.clone());

        let mut center = center;
        let reference_orbit = if !arb_precision {
            Arc::new(None)
        } else {
            Arc::new(Some(ReferenceOrbit::new(
                &self.fractal,
                match &center {
                    ComplexType::Double(_) => {
                        center = center.make_big();
                        match &center {
                            ComplexType::Double(_) => panic!("failed to make center big"),
                            ComplexType::Big(c) => c
                        }
                    },
                    ComplexType::Big(c) => c
                }, 
                self.max_iterations as usize,
                self.bailout2
            )))
        };
        
        let thread_height = dimensions.y / THREADS;

        for t in 0..THREADS {
            let renderer = Renderer {
                fractal: fractal.clone(),
                dimensions: dimensions.clone(),
                start_y: t * thread_height,
                thread_height,
                center: center.clone(),
                pixel_step: pixel_step.clone(),
                max_iterations: self.max_iterations.clone() as u32,
                bailout2: self.bailout2.clone(),
                image: Arc::clone(&image),
                layers: self.layers.clone(),
                quality,
                thread_cancel: Arc::clone(&self.thread_cancel),
                reference_orbit: Arc::clone(&reference_orbit),
                progress_tracker: Arc::clone(&progress_tracker),
                can_cancel
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

        // draw_text(
        //     &get_fps().to_string(),
        //     start_x + 30., 
        //     10., 
        //     20., 
        //     BLACK
        // );
        // draw_text(
        //     &self.quality.to_string(),
        //     start_x + 60.,
        //     10.,
        //     20.,
        //     BLACK
        // );
        // draw_text(
        //     &self.max_iterations.to_string(),
        //     screen_width() - 50., 
        //     10., 
        //     20., 
        //     BLACK
        // );
    }

    fn update(&mut self) {
        if self.exporter.exporting {
            self.exporter.update(&self.thread_pool);
            return;
        }
        if self.video_recorder.exporting {
            let timestamp = self.video_recorder.update(&self.thread_pool);

            let timestamp = match timestamp {
                None => return,
                Some(ts) => ts
            };
            self.load_timestamp(&timestamp);
            self.generate_given_image(
                Arc::clone(&self.video_recorder.image), 
                self.video_recorder.dims.clone(), 
                None,
                timestamp.pixel_step, 
                None,
                1,
                Arc::clone(&self.video_recorder.progress_tracker),
                true
            );
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

        let old_real = center.real.clone();
        center.real += movement.clone() * FBig::try_from(match (is_key_down(KeyCode::A), is_key_down(KeyCode::D)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => {moved_x = false; 0.0}
        }).unwrap();
        if moved_x && old_real == center.real {
            center.real = center.real.with_precision(old_real.precision()+1).value();
        }

        let old_im = center.im.clone();
        center.im += movement * FBig::try_from(match (is_key_down(KeyCode::W), is_key_down(KeyCode::S)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => {moved_y = false; 0.0}
        }).unwrap();
        if moved_y && old_im == center.im {
            center.im = center.im.with_precision(old_im.precision()+1).value();
        }
        
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
        let pixel_step = self.exporter.start_export(
            name, dimensions,  
            &self.current_dimensions, 
            self.pixel_step,
            VisualiserParams::get_params(&self)
        );
        let old_pixel_step = self.pixel_step;
        self.pixel_step = pixel_step;
        self.update_precision();
        self.pixel_step = old_pixel_step;

        self.generate_given_image(
            Arc::clone(&self.exporter.image), 
            self.exporter.dims.clone(), 
            None,
            pixel_step,
            None,
            1,
            Arc::clone(&self.exporter.progress_tracker),
            true
        );
    }

    fn start_recording(&mut self, name: &String, dimensions: ScreenDimensions, time: usize, fps: usize) {
        self.cancel_current_render();
        self.video_recorder.start_export(
            name, 
            dimensions, &self.current_dimensions, 
            time, fps
        );
    }

    fn pause_recording(&mut self) {
        self.video_recorder.cancel_export();
        self.cancel_current_render();
    }

    fn is_exporting(&self) -> bool {
        self.exporter.exporting || self.video_recorder.exporting
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

        if (self.moving || self.video_recorder.previewing) && self.quality < MAX_QUALITY {
            self.quality += 1;
            self.cancel_current_render();
        }
    }

    /// lets the user change the view
    pub fn user_move(&mut self) {
        if self.video_recorder.previewing { return }

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
        if self.pixel_step <= ARB_PRECISION_THRESHOLD && !self.arb_precision {
            self.arb_precision = true;
            self.center = self.center.make_big();
            self.quality += 1 ;
        } else if self.pixel_step > ARB_PRECISION_THRESHOLD && self.arb_precision {
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

    #[test]
    fn timestamp_boundary() {
        let visualiser = Visualiser::new(Fractal::Mandelbrot, 0.005, 500., 4.5, (600, 600), Layers::new(vec![Layer::default()], true));
        let mut recorder = VideoRecorder::new();
        let mut new_timestamp = VideoTimestamp::new(&visualiser, 0.);
        recorder.new_timestamp(&mut new_timestamp);
        recorder.new_timestamp(&mut new_timestamp);

        assert!(recorder.get_timestamp_at_percent(0.).is_some());
        assert!(recorder.get_timestamp_at_percent(1.).is_some());
    }
}