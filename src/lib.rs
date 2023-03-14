use macroquad::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ops::Add;
use ::rand::Rng;

pub const WIDTH: usize = 600;
pub const HEIGHT: usize = 600;
pub static BAILOUT: f64 = 4f64;
pub static ZOOM_PERCENT_INC: f64 = 0.5f64;
pub static MAX_ITER_INC_SPEED: f32 = 10f32;
pub static PALLETE_LENGTH_INC_SPEED: f32 = 50f32;
pub const THREADS: usize = 15; //12 14 15 17
pub const START_PALLETE_LENGTH: f32 = 250.;
// pub const COLOUR_MAP: [Color; 3] = [
//     DARKBLUE,
//     PINK,
//     WHITE
// ];
// pub const COLOUR_MAP: [Color; 8] = [
//     RED,
//     ORANGE,
//     YELLOW,
//     GREEN,
//     BLUE,
//     PURPLE,
//     PINK,
//     RED
// ];
// pub const COLOUR_MAP: [Color; 3] = [
//     BLACK,
//     WHITE,
//     BLACK
// ];
// pub const COLOUR_MAP: [Color; 5] = [
//     WHITE,
//     PINK,
//     BLUE,
//     PINK,
//     WHITE
// ];
pub const COLOUR_MAP: [Color; 10] = [
    WHITE,
    DARKBLUE,
    BLUE,
    DARKBROWN,
    BROWN,
    ORANGE,
    YELLOW,
    RED,
    PINK,
    WHITE
];
// pub const COLOUR_MAP: [Color; 6] = [
//     WHITE,
//     YELLOW,
//     RED,
//     DARKBLUE,
//     BLUE,
//     WHITE
// ];

fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 2..n+1 {
        result *= i;
    }
    result
}

fn choose(n: u32, r: u32) -> u32 {
    assert!(n >= r);
    factorial(n) / (factorial(r)*factorial(n-r))
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Complex {
    real: f64,
    im: f64,
}
impl Complex {
    pub fn new(real: f64, im: f64) -> Complex {
        Complex { real, im }
    }

    pub fn square(&self) -> Self {
        Complex::new(
            self.real*self.real - self.im*self.im, 
            2f64*self.real*self.im
        )
    }

    /// raise the complex number to a given power
    pub fn pow(&self, n: u32) -> Self {
        let (mut real , mut im) = (0., 0.);
        for i in 0..=n {
            let b_pow = n-i;
            let coefficient = choose(n, i) as f64 * self.im.powi(b_pow as i32) * self.real.powi(i as i32);
            match b_pow % 4 {
                0 => {real += coefficient},
                1 => {im += coefficient},
                2 => {real -= coefficient},
                3 => {im -= coefficient},
                _ => {}
            }
        }

        Complex::new( real, im )
    }

    pub fn abs_squared(&self) -> f64 {
        self.real.powi(2) + self.im.powi(2)
    }
}
impl Add for Complex { 
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            im: self.im + other.im,
        }
    }
}

/// returns the number of steps it took to diverge
/// (0 if it didn't escape)
fn diverges(c: Complex, max_iterations: u32) -> f64 {
    let mut z = Complex::new(0f64, 0f64);
    for i in 0..max_iterations {
        z = z.square() + c;
        if z.abs_squared() > BAILOUT {
            // return i as f64;
            let log_zmod = f64::log2(z.abs_squared()) / 2.0;
            let nu = f64::log2(log_zmod);
            let smooth_iteration = i as f64 + 1.0 - nu;
            return smooth_iteration;
        }  
    }
    0.0
}

#[allow(dead_code)]
const SEED: Complex = Complex { real: 0.285 , im: 0. };
#[allow(dead_code)]
/// julia set divergence
fn diverges_julia(c: Complex, max_iterations: u32) -> f64 {
    let mut z = c;
    for i in 0..max_iterations {
        z = z.square() + SEED;
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

fn complex_to_screen(pixel_step: f64, center: (f64, f64), c: Complex) -> Option<(usize, usize)> {
    let dx = c.real - center.0;
    let dy = c.im - center.1;

    let x_move = dx / pixel_step;
    let y_move = dy / pixel_step;

    let x = ((WIDTH as f64 / 2.) + x_move).floor();
    let y = ((HEIGHT as f64 / 2.) - y_move).floor();

    if x >= WIDTH as f64 || y >= HEIGHT as f64 || x < 0. || y < 0. {
        return None;
    }

    Some((x as usize, y as usize))
}

pub struct Visualiser {
    center: (f64, f64),
    pixel_step: f64,
    max_iterations: f32,
    image: Arc<Mutex<Image>>,
    texture: Texture2D, 
    move_speed: f64,
    pallete: Vec<Color>,
    pallete_length: f32
}
#[allow(dead_code)]
impl Visualiser {
    pub fn new(pixel_step: f64 , max_iterations: f32) -> Visualiser {
        Visualiser { pixel_step, max_iterations,
            center: (-0.5, 0.0),
            image: Arc::new(Mutex::new(
                Image::gen_image_color(WIDTH as u16, HEIGHT as u16, Color::new(0.0, 0.0, 0.0, 1.0)
            ))),
            texture: Texture2D::empty(),
            move_speed: 1f64, 
            pallete: Vec::new(), pallete_length: START_PALLETE_LENGTH
        }
    }

    pub fn load(&mut self, pixel_step: f64, center_x: f64, center_y: f64) {
        self.pixel_step = pixel_step;
        self.center = (center_x, center_y);
    }

    /// returns the number of steps it took to diverge
    /// (0 if it didn't escape)
    fn diverges(&self, c: Complex) -> f64 {
        let mut z = Complex::new(0f64, 0f64);
        for i in 0..self.max_iterations as u32 {
            z = z.square() + c;
            if z.abs_squared() > BAILOUT {
                let log_zmod = f64::log2(z.abs_squared()) / 2.0;
                let nu = f64::log2(log_zmod);
                let smooth_iteration = i as f64 + 1.0 - nu;
                // let smooth_iteration = i as f64 - f64::log2(f64::max(1.0, log_zmod));
                return smooth_iteration;
            }  
        }
        0.0
    }

    fn lerp(&self, a: f32, b: f32, t: f32) -> f32 {
        (1f32 - t) * a + t * b
    }

    fn interpolate_colour(&self, c1: Color, c2: Color, fraction: f32) -> Color {
        Color::new(
            self.lerp(c1.r, c2.r, fraction),
            self.lerp(c1.g, c2.g, fraction),
            self.lerp(c1.b, c2.b, fraction),
            1.0
        )
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
            let lower_i = self.world_index_to_i(lower_world_index);
            let upper = COLOUR_MAP[colour_index+1];
            let upper_i = self.world_index_to_i(lower_world_index+1.);

            let inner_fraction = (i-lower_i) as f32 / (upper_i-lower_i) as f32;

            self.pallete.push(self.interpolate_colour(lower, upper, inner_fraction));
        }
    }

    fn escape_time(&self, diverge_num: f64) -> Color {
        if diverge_num == 0. {
            return BLACK;
        }

        let lower_colour = self.pallete[diverge_num.floor() as usize];
        let upper_colour = self.pallete[(diverge_num.floor() as usize + 1).min(self.pallete.len()-1)];
        
        self.interpolate_colour(lower_colour, upper_colour, diverge_num as f32 % 1.0)
    }
    
    fn exponentially_mapped(&self, diverge_num: f64) -> Color {
        let n = (diverge_num as f32 + 1.0).ln() / self.max_iterations.ln();
        let c1 = BLACK;
        let c2 = PINK;

        Color::new(c1.r + (c2.r - c1.r) * n, c1.g + (c2.g - c1.g) * n, c1.b + (c2.b - c1.b) * n, 1.0)
    }

    /// generates and stores the mandlebrot image
    /// for the current hyperparameters
    pub fn generate_image(&mut self) {
        if self.pallete.len() != self.max_iterations as usize {
            self.make_pallete();
        }
        
        let thread_height = HEIGHT / THREADS;
        let mut threads = Vec::with_capacity(THREADS);
        for t in 0..THREADS {
            let start_y = t*thread_height;
            let image = Arc::clone(&self.image);
            let pixel_step = self.pixel_step.clone();
            let center = self.center.clone();
            let max_iterations = self.max_iterations.clone() as u32;
            let pallete = self.pallete.clone();
            threads.push(thread::spawn(move || {
                for x in 0..WIDTH {
                    for y in start_y..start_y+thread_height {
                        let z = Complex::new(
                            (center.0 - WIDTH as f64/2.0 * pixel_step) + x as f64 * pixel_step, 
                            (center.1 - HEIGHT as f64/2.0 * pixel_step) + y as f64 * pixel_step,
                        );
                        let diverge_num = diverges(z, max_iterations);
                        let colour = escape_time(diverge_num, &pallete);
                        
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

        let size = WIDTH as f32 / self.pallete.len() as f32;
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

    /// lets the user move the view
    pub fn user_move(&mut self) {
        let (mut moved_y, mut moved_x) = (true, true);
        let (mut zoomed, mut iter, mut pallete, mut tp) = (true, true, true, false);
        let dt = get_frame_time() as f64;

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

        self.max_iterations += MAX_ITER_INC_SPEED * dt as f32 * match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => {iter = false; 0.0}
        };

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

        if is_key_pressed(KeyCode::Z) {
            println!("{} {} = {}x zoom\n{:?}", self.max_iterations, self.pixel_step, 0.005/self.pixel_step, self.center);
        }
    
        // teleport to the top
        if is_key_pressed(KeyCode::T) {
            self.pixel_step = 0.005;
            self.move_speed = 1.;
            tp = true;
        }
    
        if moved_y || moved_x || zoomed || iter || pallete || tp {
            self.generate_image();
        }
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

    /// loads the buhddabrot channel onto the main image
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
                        if let Some(p) = complex_to_screen(pixel_step, center, z) {
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
    fn add() {
        let a = Complex::new(1f64, 2f64);
        let b = Complex::new(3f64, 4f64);
        assert_eq!(a + b, Complex::new(4f64, 6f64));
    }

    #[test]
    fn sqauare() {
        let a = Complex::new(1f64, 2f64);
        assert_eq!(a.square(), Complex::new(-3f64, 4f64));
    }

    #[test]
    fn converges() {
        let a = Complex::new(0f64, 0f64);
        let visualiser = Visualiser::new(2.2, 50.0);

        assert_eq!(0.0, visualiser.diverges(a));
    }

    #[test]
    fn diverges() {
        let a = Complex::new(-1.0, 1.0);
        let visualiser = Visualiser::new(2.2, 50.0);

        assert!(visualiser.diverges(a) > 0.0);
    }

    #[test]
    fn power() {
        let a = Complex::new( 2., -5. );
        let a3 = a.pow(3);

        assert_eq!(Complex::new(-142., 65.), a3);
    }

    #[test]
    fn complex_screen() {
        let c = Complex::new(-0.5, 0.0);
        let screen = Some((WIDTH / 2, HEIGHT / 2));

        assert_eq!(screen, complex_to_screen(0.005, (c.real, c.im), c));
    }
}