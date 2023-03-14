use macroquad::prelude::*;
use std::time::Instant;

#[allow(unused_imports)]
use mandlebrot::Visualiser;
#[allow(unused_imports)]
use mandlebrot::Buhddabrot;

#[macroquad::main("mandlebrot")]
async fn main() {
    request_new_screen_size(mandlebrot::WIDTH as f32, mandlebrot::HEIGHT as f32);
    next_frame().await;

    // let mut visualiser = Visualiser::new(0.005, 500.0);
    let mut visualiser = Buhddabrot::new(0.005, 2_500., 50_000_000, true);

    // visualiser.load(0.002, -1.15, 0.);
    
    let now = Instant::now();
    visualiser.generate_image();
    println!("Took {} seconds to generate",  now.elapsed().as_secs_f32());

    loop {
        visualiser.draw();
        visualiser.user_move();

        next_frame().await
    }
}