use macroquad::prelude::*;
use std::time::Instant;

use mandlebrot::ScreenDimensions;
use mandlebrot::orbit_trap::*;
use mandlebrot::layers::*;
use mandlebrot::palletes::*;

#[allow(unused_imports)]
use mandlebrot::Visualiser;
#[allow(unused_imports)]
use mandlebrot::Buhddabrot;
#[allow(unused_imports)]
use mandlebrot::JuliaSeed;

#[macroquad::main("mandelbrot")]
async fn main() {
    request_new_screen_size(mandlebrot::WIDTH as f32, mandlebrot::HEIGHT as f32);
    next_frame().await;

    let mut visualiser = Visualiser::new(
        0.005, 
        500.0, 
        (600, 600),
        ScreenDimensions::tuple_4k(),
        Layers::new(vec![
            Layer::new(LayerType::Colour, 1.0, ICE.to_vec(), 153.173),
            // Layer::new(LayerType::ColourOrbitTrap(OrbitTrapType::Cross(OrbitTrapCross::new((2.5, 2.5), 5.0))), 
            //            0.5, vec![WHITE, PINK, RED, WHITE], 250.0),
            Layer::new(LayerType::Shading3D, 1.0, vec![], 1000.0),
            Layer::new(LayerType::Shading, 1.0, 
                       vec![WHITE, WHITE, WHITE, BLACK], 18.0),
            Layer::new(LayerType::ShadingOrbitTrap(OrbitTrapType::Circle(OrbitTrapCircle::new((0.0, 0.0), 10.0))), 
                       1.0, vec![WHITE, WHITE, BLACK], 1000.0)
        ])
    );

    // let mut visualiser = Buhddabrot::new(0.005, 2_500., 50_000_000, true);

    // visualiser.load(0.002, -1.15, 0., 2_500.);
    // visualiser.load((0.308400934550109715351-0.308400934548031413847)/600., 0.5*(0.308400934550109715351+0.308400934548031413847), 0.5*(0.0252645634954311029242+0.0252645634938723767962), 2500.0);
    // visualiser.load(0.005, 0.3080738405277603, 0.022720381308498527, 600.1);
    // visualiser.load( 0.000000000007506999894187192, -1.7478569335479708, 2.0020200029685542e-5, 500.);
    // visualiser.load(0.000000000012544813697480459, -1.7857173222072602, 6.485835101156323e-5, 2000.);
    // visualiser.load(0.00000000027267392679500867, -1.7588897614644763, -0.019085635569219, 2000.0);
    // visualiser.load(0.000000000000040685052352904394, -1.758889768238364, -0.01908561816083628, 3000.0);
    // visualiser.load(0.005, -1.7492892108246816, 3.46877435179622e-6, 500.0);

    let now = Instant::now();
    visualiser.generate_image();
    println!("Took {} seconds to generate",  now.elapsed().as_secs_f32());

    loop {
        visualiser.draw();
        visualiser.user_move();
        // visualiser.play(0.4);

        next_frame().await
    }
}