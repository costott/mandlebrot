use macroquad::prelude::*;
use chrono::format::strftime::StrftimeItems;
use std::sync::{Arc, Mutex};
use std::thread;
use std::f64::consts::PI;
use std::slice::Iter;
use std::fs;
use ::rand::Rng;

mod complex;
use complex::Complex;
mod palletes;
use palletes::COLOUR_MAP;

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

// user changing view
pub const ZOOM_PERCENT_INC: f64 = 0.5f64;
pub const MAX_ITER_INC_SPEED: f32 = 10f32;
pub const PALLETE_LENGTH_INC_SPEED: f32 = 50f32;

pub const THREADS: usize = 15; //12 14 15 17

pub const START_PALLETE_LENGTH: f32 = 189.43372;//250.;

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

#[derive(Clone)]
pub enum RenderMode {
    ColouredFlat,
    ColouredFlatDerivatived,
    Greyscale3D,
    Coloured3D,
    Greyscale3DVariated,
    Julia(JuliaSeed)
}
impl RenderMode {
    /// provides an iterator for the render modes
    /// used when generating images
    pub fn iter_image() -> Iter<'static, RenderMode> {
        static RENDERMODES: [RenderMode; 3] = [
            RenderMode::ColouredFlat,
            RenderMode::Coloured3D,
            RenderMode::Greyscale3DVariated
        ];
        RENDERMODES.iter()
    }

    pub fn as_string(render_mode: &RenderMode) -> String {
        String::from( match render_mode {
            RenderMode::ColouredFlat => "ColouredFlat", 
            RenderMode::ColouredFlatDerivatived => "ColouredFlatDerivatived", 
            RenderMode::Greyscale3D => "Greyscale3D", 
            RenderMode::Coloured3D => "Coloured3D",
            RenderMode::Greyscale3DVariated => "Greyscale3DVariated",
            RenderMode::Julia(_) => "Julia"
        })
    }
}

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
    if t == 0. {
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

pub struct Visualiser {
    view_dimensions: ScreenDimensions,
    screenshot_dimensions: ScreenDimensions,
    current_dimensions: ScreenDimensions,
    center: (f64, f64),
    pixel_step: f64,
    max_iterations: f32,
    render_mode: RenderMode,
    image: Arc<Mutex<Image>>,
    texture: Texture2D, 
    move_speed: f64,
    pallete: Vec<Color>,
    pallete_length: f32
}
impl Visualiser {
    pub fn new(
        pixel_step: f64, 
        max_iterations: f32, 
        render_mode: RenderMode,
        view_dimensions: (usize, usize),
        screenshot_dimensions: (usize, usize)
    ) -> Visualiser {
        Visualiser { pixel_step, max_iterations, render_mode, 
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
            pallete: Vec::new(), pallete_length: START_PALLETE_LENGTH
        }
    }

    pub fn load(&mut self, pixel_step: f64, center_x: f64, center_y: f64, max_iterations: f32) {
        self.pixel_step = pixel_step;
        self.center = (center_x, center_y);
        self.max_iterations = max_iterations;
    }

    fn i_to_world_index(&self, i: u32) -> f32 {
        (i as f32 / self.max_iterations) * (1000. / self.pallete_length) * (COLOUR_MAP.len()-1) as f32
    }

    fn world_index_to_i(&self, world_index: f32) -> u32 {
        (world_index * (1. / (COLOUR_MAP.len()-1) as f32) * (self.pallete_length / 1000.) * self.max_iterations).floor() as u32
    }

    fn make_pallete(&mut self) {
        self.pallete = Vec::with_capacity(self.max_iterations as usize);

        for i in 0..=self.max_iterations as u32 {
            let world_index = self.i_to_world_index(i);
            let lower_world_index = world_index.floor();
            let colour_index = world_index.floor() as usize % (COLOUR_MAP.len()-1);
            
            let lower = COLOUR_MAP[colour_index];
            // if world_index < (COLOUR_MAP.len()-1) as f32 && colour_index == 0 {
            //     lower = BLACK;
            // }
            let lower_i = self.world_index_to_i(lower_world_index);
            let upper = COLOUR_MAP[colour_index+1];
            let upper_i = self.world_index_to_i(lower_world_index+1.);

            let inner_fraction = (i-lower_i) as f32 / (upper_i-lower_i) as f32;

            self.pallete.push(interpolate_colour(lower, upper, inner_fraction));
        }
    }

    /// generates and stores the mandlebrot image
    /// for the current parameters
    pub fn generate_image(&mut self) {
        if self.pallete.len() != self.max_iterations as usize {
            self.make_pallete();
        }
        
        let thread_height = self.current_dimensions.y / THREADS;
        let mut threads = Vec::with_capacity(THREADS);
        for t in 0..THREADS {
            let start_y = t*thread_height;
            let center = self.center.clone();
            let pixel_step = self.pixel_step.clone();
            let max_iterations = self.max_iterations.clone() as u32;
            let image = Arc::clone(&self.image);
            let pallete = self.pallete.clone();
            let render_mode = self.render_mode.clone();
            let dimensions = self.current_dimensions.clone();
            threads.push(thread::spawn(move || { // nesting to infinity
                for x in 0..dimensions.x {
                    for y in start_y..start_y+thread_height {
                        let z = Complex::new(
                            (center.0 - dimensions.x as f64/2.0 * pixel_step) + x as f64 * pixel_step, 
                            (center.1 - dimensions.y as f64/2.0 * pixel_step) + y as f64 * pixel_step,
                        );
                        let colour: Color = match render_mode {
                            RenderMode::ColouredFlat => {
                                let diverge_num = diverges(z, max_iterations);
                                escape_time(diverge_num, &pallete)
                            },
                            RenderMode::ColouredFlatDerivatived => {
                                let diverge_num = diverges_der(z, max_iterations);
                                escape_time(diverge_num, &pallete)
                            }, 
                            RenderMode::Greyscale3D => {
                                let t = diverges_3d(z, max_iterations);
                                colour_3d(t, WHITE)
                            },
                            RenderMode::Greyscale3DVariated => {
                                let t = diverges_3d_variated(z, max_iterations);
                                colour_3d(t, WHITE)
                            }
                            RenderMode::Coloured3D => {
                                let (t, diverge_num) = diverges_3d_coloured(z, max_iterations);
                                let colour = escape_time(diverge_num, &pallete);
                                colour_3d(t, colour)
                            },
                            RenderMode::Julia(ref seed) => {
                                let diverge_num = diverges_julia(z, max_iterations, seed);
                                escape_time(diverge_num, &pallete)
                            }
                        };
                        
                        let mut im = image.lock().unwrap();
                        im.set_pixel(x as u32, y as u32, colour);
                    }
                }
            }));
        }
        for thread in threads {
            thread.join().unwrap();
        }

        self.texture = Texture2D::from_image(&self.image.clone().lock().unwrap());
    }

    /// draw a generated image to the screen
    pub fn draw(&self) {
        draw_texture(self.texture, 0.0, 0.0, WHITE);

        let size = self.current_dimensions.x as f32 / self.pallete.len() as f32;
        for (i, colour) in self.pallete.iter().enumerate() {
            draw_rectangle(
                i as f32 * size, 
                0., 
                size, 
                10., 
                *colour
            );
        }

        draw_text(
            &get_fps().to_string(), 
            10., 
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

        iter
    }

    /// lets the user shitf the pallete length
    /// 
    /// returns if the pallete has been changed or not
    fn user_shift_pallete(&mut self, dt: f64) -> bool {
        let mut pallete = true;

        self.pallete_length += PALLETE_LENGTH_INC_SPEED * dt as f32 * match (is_key_down(KeyCode::I), is_key_down(KeyCode::P)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => {pallete = false; 0.0}
        };
        if pallete {
            self.pallete_length = self.pallete_length.min(1000.);
            self.pallete_length = self.pallete_length.max(0.);
            self.make_pallete();
        }

        pallete
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
        fs::create_dir(&images_path).unwrap();

        let old_render_mode = self.render_mode.clone();

        // change dimensions and pixel step for higher quality
        let old_pixel_step = self.pixel_step.clone();
        let screen_height = self.view_dimensions.y as f64 * self.pixel_step;
        self.pixel_step = screen_height / self.screenshot_dimensions.y as f64;
        self.current_dimensions = self.screenshot_dimensions.clone();
        self.image = Arc::new(Mutex::new(
            Image::gen_image_color(self.current_dimensions.x as u16, self.current_dimensions.y as u16, 
                                   Color::new(0.0, 0.0, 0.0, 1.0)
        )));

        for render_mode in RenderMode::iter_image() {
            self.render_mode = render_mode.clone();
            self.generate_image();
            
            let path = &format!["images/{}/{}.png",
                image_name.clone(),
                RenderMode::as_string(render_mode)
            ];
            self.image.clone().lock().unwrap().export_png(path);
        }

        self.render_mode = old_render_mode;
        self.pixel_step = old_pixel_step;
        self.current_dimensions = self.view_dimensions.clone();
        self.image = Arc::new(Mutex::new(
            Image::gen_image_color(self.current_dimensions.x as u16, self.current_dimensions.y as u16, 
                                   Color::new(0.0, 0.0, 0.0, 1.0)
        )));

        self.generate_image();
    }

    /// lets the user move the view
    pub fn user_move(&mut self) {
        let dt = get_frame_time() as f64;

        let moved_view = self.user_move_view(dt);
        let zoomed = self.user_zoom(dt);
        let iter = self.user_change_max_iteration(dt);
        let pallete = self.user_shift_pallete(dt);
        let tp = self.user_teleport();

        if is_key_pressed(KeyCode::Z) {
            println!("{} {} = {}x zoom\n{:?}\n{}", 
                self.max_iterations, self.pixel_step, 0.005/self.pixel_step, self.center, self.pallete_length);
        }
        self.user_export();   
    
        if moved_view || zoomed || iter || pallete || tp {
            self.generate_image();
        }
    }

    /// automatically zooms into the center
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