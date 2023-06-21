// © 2023 costott. All rights reserved. 
// This code is provided for viewing purposes only. Copying, reproduction, 
// or distribution of this code, in whole or in part, in any form or by any 
// means, is strictly prohibited without prior written permission from the 
// copyright owner.

use macroquad::prelude::*;

use super::*;

fn i_to_world_index(i: u32, colour_map: &Vec<Color>, length: f32, max_iterations: f32) -> f32 {
    (i as f32 / (max_iterations+1.)) * (1000. / length) * (colour_map.len()-1) as f32
}

fn world_index_to_i(world_index: f32, colour_map: &Vec<Color>, length: f32, max_iterations: f32) -> u32 {
    (world_index * (1. / (colour_map.len()-1) as f32) * (length / 1000.) * (max_iterations+1.)).floor() as u32
}

fn make_pallete(colour_map: &Vec<Color>, length: f32, max_iterations: f32) -> Vec<Color> {
    let mut pallete = Vec::with_capacity(max_iterations as usize);

    for i in 0..=max_iterations as u32 {
        let world_index = i_to_world_index(i, colour_map, length, max_iterations);
        let lower_world_index = world_index.floor();
        // convert world index to the colour it represents in the pallete
        let colour_index = world_index.floor() as usize % (colour_map.len()-1);
        
        let lower = colour_map[colour_index];
        let lower_i = world_index_to_i(lower_world_index, colour_map, length, max_iterations);
        // UNCOMMENT IF FIRST COLOUR SHOULD BE A DIFFERENT COLOUR TO THE SCHEME
        // (NEED TO MAKE LOWER MUT)
        // if world_index < (COLOUR_MAP.len()-1) as f32 && colour_index == 0 {
        //     lower = BLACK;
        // }
        let upper = colour_map[(colour_index+1).min(colour_map.len()-1)];
        let upper_i = world_index_to_i(lower_world_index+1., colour_map, length, max_iterations);

        let inner_fraction = (i-lower_i) as f32 / (upper_i-lower_i) as f32;

        pallete.push(interpolate_colour(lower, upper, inner_fraction));
    }

    pallete
}

/// analyse the given complex number, letting the implementors
/// calculate their outputs
/// 
/// # Returns
/// returns if the point is in the set or not
fn diverges_implementors(c: ComplexType, max_iterations: u32, implementations: &mut Vec<LayerImplementation>) -> bool {
    let mut z = c.clone();
    for im in implementations.iter_mut() {
        im.before(z.clone(), max_iterations);
    }

    for i in 0..max_iterations {        
        // if z.real.abs() > BAILOUT_ORBIT_TRAP || z.im.abs() > BAILOUT_ORBIT_TRAP { <- much faster but less accurate
        if z.abs_squared() > BAILOUT_ORBIT_TRAP {
            for im in implementations.iter_mut() {
                im.out_set(z.clone(), i);
            }
            return false;
        }

        for im in implementations.iter_mut() {
            im.during(z.clone(), i);
        }

        z = z.square() + c.clone();
    }

    for im in implementations.iter_mut() {
        im.in_set(z.clone());
    }
    return true;
}

#[derive(Clone)]
/// the type of colouring algorithm used for a layer
pub enum LayerType {
    Colour,
    ColourOrbitTrap(OrbitTrapType),
    Shading,
    Shading3D,
    ShadingOrbitTrap(OrbitTrapType)
}
impl LayerType {
    /// returns whether the layer is a shading layer
    pub fn shading_layer(&self) -> bool {
        match self {
            LayerType::Shading | LayerType::Shading3D | &LayerType::ShadingOrbitTrap(_) => {
                true
            },
            _ => false
        }
    }
}

// this should be made for generation not by the user
/// Controls the behaviour of layer implementors when analysing a point
trait LayerImplementor {
    /// what needs to happen before iterations start for a given pixel
    fn before(&mut self, z: ComplexType, max_iterations: u32);
    /// what needs to happen each iteration, before the next z is calculated
    fn during(&mut self, z: ComplexType, i: u32);
    /// what needs to happen if the point is outside the set
    fn out_set(&mut self, z: ComplexType, i: u32);
    /// what needs to happen if the point is inside the set
    fn in_set(&mut self, z: ComplexType);
    /// output the generated value
    fn get_output(&self) -> f64;
    /// converts all numbers used to arbitrary precision
    fn make_big(&mut self) {}
    /// converts all numbers used to double precision
    fn make_double(&mut self) {}
}

// idk how to do polymorphism well so this is my best shot 
// sorry if it's awful I don't want to use box dyns 
#[derive(Clone)]
/// Implementors which are used during iteration to calculate
/// the values needed for the layers, storing the output in the 
/// implementation.
enum LayerImplementation {
    ColourImplemetor(ColourImplemetor),
    OrbitTrapImplementor(OrbitTrapImplementor),
    Shading3DImplementor(Shading3DImplementor)
}
impl LayerImplementor for LayerImplementation {
    fn before(&mut self, z: ComplexType, max_iterations: u32) {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.before(z, max_iterations),
            LayerImplementation::OrbitTrapImplementor(im) => im.before(z, max_iterations),
            LayerImplementation::Shading3DImplementor(im) => im.before(z, max_iterations)
        }
    }

    fn during(&mut self, z: ComplexType, i: u32) {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.during(z, i),
            LayerImplementation::OrbitTrapImplementor(im) => im.during(z, i),
            LayerImplementation::Shading3DImplementor(im) => im.during(z, i)
        }
    }

    fn out_set(&mut self, z: ComplexType, i: u32) {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.out_set(z, i),
            LayerImplementation::OrbitTrapImplementor(im) => im.out_set(z, i),
            LayerImplementation::Shading3DImplementor(im) => im.out_set(z, i)
        }
    }

    fn in_set(&mut self, z: ComplexType) {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.in_set(z),
            LayerImplementation::OrbitTrapImplementor(im) => im.in_set(z),
            LayerImplementation::Shading3DImplementor(im) => im.in_set(z)
        }
    }

    fn get_output(&self) -> f64 {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.get_output(),
            LayerImplementation::OrbitTrapImplementor(im) => im.get_output(),
            LayerImplementation::Shading3DImplementor(im) => im.get_output()
        }
    }

    fn make_big(&mut self) {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.make_big(),
            LayerImplementation::OrbitTrapImplementor(im) => im.make_big(),
            LayerImplementation::Shading3DImplementor(im) => im.make_big()
        }
    }

    fn make_double(&mut self) {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.make_double(),
            LayerImplementation::OrbitTrapImplementor(im) => im.make_double(),
            LayerImplementation::Shading3DImplementor(im) => im.make_double()
        }
    }
}

#[derive(Clone)]
/// basic colour algorithm tracking the number of iterations a point took to diverge,
/// calculating the smooth iteration
struct ColourImplemetor {
    output: f64
}
impl ColourImplemetor {
    fn new() -> ColourImplemetor {
        ColourImplemetor { output: 0.0 }
    }
}
impl LayerImplementor for ColourImplemetor {
    fn before(&mut self, _z: ComplexType, _max_iterations: u32) {}

    fn during(&mut self, _z: ComplexType, _i: u32) {}

    fn out_set(&mut self, z: ComplexType, i: u32) {
        let log_zmod = f64::log2(z.abs_squared()) / 2.0;
        let nu = f64::log2(log_zmod);
        let smooth_iteration = i as f64 + 1.0 - nu;
        self.output =  smooth_iteration;
    }

    fn in_set(&mut self, _z: ComplexType) {
        self.output = 0.0;
    }

    fn get_output(&self) -> f64 {
        self.output
    }
}

#[derive(Clone)]
/// orbit trapped algorithm looking at the minimum distance between an orbit and a trap,
/// calculating a trapped index to be used in the pallete
struct OrbitTrapImplementor {
    output: f64,
    min_distance2: f64,
    divisor: f64,
    trap: OrbitTrapType,
    /// 'vector' of closest point to the trap
    closest_to_trap: ComplexType
}
impl OrbitTrapImplementor {
    fn new(trap: OrbitTrapType) -> OrbitTrapImplementor {
        OrbitTrapImplementor { 
            output: 0.0, 
            min_distance2: trap.greatest_distance2(),
            divisor: 0.0, 
            trap,
            closest_to_trap: ComplexType::Double(Complex::new(0.0, 0.0))
        }
    }

    fn generate_output(&self) -> f64 {
        let output = match self.trap.get_analysis() {
            OrbitTrapAnalysis::Distance => self.min_distance2.sqrt(),
            OrbitTrapAnalysis::Real => self.closest_to_trap.real_f64().abs(),
            OrbitTrapAnalysis::Imaginary => self.closest_to_trap.im_f64().abs(),
            OrbitTrapAnalysis::Angle => PI + self.closest_to_trap.arg()
        } / self.divisor;
        output
    }
}
impl LayerImplementor for OrbitTrapImplementor {
    fn before(&mut self, _z: ComplexType, max_iterations: u32) {
        self.divisor = self.min_distance2.sqrt() / max_iterations as f64;
    }

    fn during(&mut self, z: ComplexType, _i: u32) {
        // if i == 0 {return}
        let z_trap_distance2 = self.trap.distance2(z.clone());
        if z_trap_distance2 < self.min_distance2 {
            self.min_distance2 = z_trap_distance2;
            self.closest_to_trap = self.trap.vector(z);
        }
    }

    fn out_set(&mut self, _z: ComplexType, _i: u32) {
        self.output = self.generate_output();
    }

    fn in_set(&mut self, _z: ComplexType) {
        self.output = self.generate_output();
    }

    fn get_output(&self) -> f64 {
        self.output
    }

    fn make_big(&mut self) {
        self.closest_to_trap.make_big();
    }

    fn make_double(&mut self) {
        self.closest_to_trap.make_double();
    }
}

#[derive(Clone)]
/// 3d algorithm to shade the set to give height.
/// Theory from: https://www.math.univ-toulouse.fr/~cheritat/wiki-draw/index.php/Mandelbrot_set#Normal_map_effect,
/// calculating a t value that represents darkness/brightness
struct Shading3DImplementor {
    output: f64,
    v: ComplexType,
    der: ComplexType,
    dc: ComplexType,
}
impl Shading3DImplementor {
    fn new() -> Shading3DImplementor {
        Shading3DImplementor { 
            output: 0.0, 
            v: ComplexType::Double(Complex::new(
                f64::cos(ANGLE * (PI / 180.)),
                f64::sin(ANGLE * (PI / 180.))
            )),
            der: ComplexType::Double(Complex::new(1., 0.)), 
            dc: ComplexType::Double(Complex::new(1., 0.)),
        }
    }

    fn generate_output(&self, z: ComplexType) -> f64 {
        let mut u = z / self.der.clone();
        u = u.clone() / f64::sqrt(u.abs_squared());
        let t = u.real_f64()*self.v.real_f64() + u.im_f64()*self.v.im_f64();
        let mut t = t+ H2;
        t = t/(1.+H2);
        t = f64::max(0.1, t);
        t
    }
}
impl LayerImplementor for Shading3DImplementor {
    fn before(&mut self, _z: ComplexType, _max_iterations: u32) {}

    fn during(&mut self, z: ComplexType, _i: u32) {
        self.der = self.der.clone() * (z * 2.) + self.dc.clone();
    }

    fn out_set(&mut self, z: ComplexType, _i: u32) {
        self.output = self.generate_output(z);
    }

    fn in_set(&mut self, z: ComplexType) {
        self.output = self.generate_output(z);
    }

    fn get_output(&self) -> f64 {
        self.output
    }

    fn make_big(&mut self) {
        self.v = self.v.make_big();
        self.der = self.der.make_big();
        self.dc = self.dc.make_big();
    }

    fn make_double(&mut self) {
        self.v = self.v.make_double();
        self.der = self.der.make_double();
        self.dc = self.dc.make_double();
    }
}

/// creates the implementors which will be 
/// used to calculate values during iteration
/// 
/// # Returns
/// 
/// Vector of implementations, and a vector of indexes which map
/// each layer to the index of the implementation it needs to use
fn make_implementors(layers: &Vec<Layer>) -> (Vec<LayerImplementation>, Vec<usize>) {
    let mut implementors = Vec::new();
    // colour and shading3D only need to be added once, so this
    // keeps track of the index of the implementors
    let mut colour_in: i16 = -1;
    let mut shading3d_in: i16 = -1;

    let mut implementor_map = Vec::with_capacity(layers.len());
    for layer in layers {
        match &layer.layer_type {
            LayerType::Colour | LayerType::Shading => {
                if colour_in == -1 {
                    implementors.push(LayerImplementation::ColourImplemetor(ColourImplemetor::new()));
                    colour_in = (implementors.len()-1) as i16;
                }
                implementor_map.push(colour_in as usize);
            },
            LayerType::Shading3D => {
                if shading3d_in == -1 {
                    implementors.push(LayerImplementation::Shading3DImplementor(Shading3DImplementor::new()));
                    shading3d_in = (implementors.len()-1) as i16;
                }
                implementor_map.push(shading3d_in as usize);
            },
            LayerType::ColourOrbitTrap(trap) | LayerType::ShadingOrbitTrap(trap) => {
                implementors.push(LayerImplementation::OrbitTrapImplementor(OrbitTrapImplementor::new(trap.clone())));
                implementor_map.push(implementors.len()-1);
            }
        }
    }

    (implementors, implementor_map)
}

/// A collection of layers that will be used to colour the set
/// 
/// The order of layers is important - it is the order of application
#[derive(Clone)]
pub struct Layers {
    pub layers: Vec<Layer>,
    implementors: Vec<LayerImplementation>,
    implementor_map: Vec<usize>,
    pub arb_precision: bool
}
impl Layers {
    /// Create new layers for rendering
    /// 
    /// # Panics
    /// no layers are given (empty vector)
    /// 
    /// the first layer is a shading layer
    /// 
    /// a shading layer is being applied to a part of the set
    /// with no other layers acting in the same layer
    pub fn new(layers: Vec<Layer>) -> Layers {
        // has to be multiple layers
        if layers.len() == 0 {
            panic!("there must be at least 1 layer");
        }

        // the first layer can't be a shading layer
        if match layers[0].layer_type {
            LayerType::Shading | LayerType::Shading3D |
            LayerType::ShadingOrbitTrap(_) => 1,
            _ => 0
        } == 1 {
            panic!("the first layer can't be a shading layer")
        }

        let mut non_shade_in_set = false;
        let mut non_shade_out_set = false;
        for layer in layers.iter() {
            if layer.layer_type.shading_layer() { 
                if layer.layer_range.layer_covered(non_shade_in_set, non_shade_out_set) { continue }
                panic!("Shading layer exists that isn't covered by another layer");
            }
            match layer.layer_range {
                LayerRange::Both => {
                    non_shade_in_set = true;
                    non_shade_out_set = true;
                },
                LayerRange::InSet => {
                    non_shade_in_set = true;
                },
                LayerRange::OutSet => {
                    non_shade_out_set = true;
                }
            }
        }

        let (implementors, implementor_map) = make_implementors(&layers);
        
        Layers { layers, implementors, implementor_map, arb_precision: false }
    }

    /// makes all the numbers used in the implementors arbitary precision
    pub fn make_big(&mut self) {
        for implementor in self.implementors.iter_mut() {
            implementor.make_big();
        }
        self.arb_precision = true;
    }

    /// makes all the numbers used in the implementors double precision
    pub fn make_double(&mut self) {
        for implementor in self.implementors.iter_mut() {
            implementor.make_double();
        }
        self.arb_precision = false;
    }

    /// makes sure all the palletes for the layers
    /// are updated for the current max iterations
    pub fn generate_palletes(&mut self, max_iterations: f32) {
        for layer in self.layers.iter_mut() {
            layer.generate_pallete(max_iterations);
        }
    }

    /// get the colour for the given complex number after passing 
    /// through all the layers
    pub fn colour_pixel(&self, c: ComplexType, max_iterations: u32) -> Color {
        self.colour_pixel_implementors(c, max_iterations)
    }

    fn colour_pixel_implementors(&self, c: ComplexType, max_iterations: u32) -> Color {
        let mut implementors = self.implementors.clone();
        let in_set = diverges_implementors(c, max_iterations, &mut implementors);

        let mut colour: Option<Color> = None;
        for (i, layer) in self.layers.iter().enumerate() {
            let output = implementors[self.implementor_map[i]].get_output();
            colour = layer.colour_implementors(colour, output, in_set);
        }

        match colour {
            Some(c) => c,
            None => BLACK
        }
    }
}

#[derive(Clone)]
/// specifies the range of the mandelbrot set a layer is applied to
pub enum LayerRange {
    InSet,
    OutSet,
    Both
}
impl LayerRange {
    /// returns whether they layer applies to a point in/out the set
    pub fn layer_applies(&self, in_set: bool) -> bool {
        match self {
            LayerRange::InSet => {return in_set},
            LayerRange::OutSet => {return !in_set},
            LayerRange::Both => {return true}
        }
    }

    // returns whether the layer is covered by another layer already
    fn layer_covered(&self, covered_in_set: bool, covered_out_set: bool) -> bool {
        match self {
            LayerRange::Both => {covered_in_set && covered_out_set},
            LayerRange::InSet => {covered_in_set},
            LayerRange::OutSet => {covered_out_set}
        }
    }
}

/// A colouring layer for the mandelbrot set
#[derive(Clone)]
pub struct Layer {
    pub layer_type: LayerType,
    pub layer_range: LayerRange,
    pub strength: f32,
    colour_map: Vec<Color>,
    pallete_length: f32,
    pub pallete: Vec<Color>
}
impl Layer {
    /// # Params
    /// `layer_type`: the type of colouring it does
    /// 
    /// `strength`: how much of the current colour generated by the previous layers should it override
    /// **0.0 => None, 1.0 => All**
    /// 
    /// `colour_map`: an outline of the main colours used in the colour scheme
    /// **THIS IS NOT NEEDED IF THE LAYERTYPE IS SHADING3D**
    /// 
    /// `pallete_length`: how long 1 repetition of the colour map should be in the 1000.0 length pallete
    pub fn new(
        layer_type: LayerType, 
        layer_range: LayerRange, 
        strength: f32, 
        colour_map: Vec<Color>, 
        pallete_length: f32
    ) -> Layer {
        Layer {
            layer_type, layer_range, strength, colour_map, pallete_length,
            pallete: Vec::new()
        }
    }

    pub fn generate_pallete(&mut self, max_iterations: f32) {
        match self.layer_type {
            LayerType::Shading3D => {return},
            _ => {}
        }
        // idk why the +1 is needed here but not before but it doesn't work without it 
        if self.pallete.len() == max_iterations as usize + 1 {return}
        self.pallete = make_pallete(&self.colour_map, self.pallete_length, max_iterations);
    }

    pub fn change_pallete_length(&mut self, change: f32, max_iterations: f32) {
        self.pallete_length += change;
        self.pallete_length = f32::min(f32::max(0., self.pallete_length), 1000.);
        self.pallete = make_pallete(&self.colour_map, self.pallete_length, max_iterations)
    }

    pub fn get_pallete_length(&self) -> f32 {
        self.pallete_length
    } 

    /// calculate the colour for the Colour layer type
    fn colour(&self, diverge_num: f64) -> Color {
        escape_time(diverge_num, &self.pallete)
    }

    /// calculate the colour for the Shading layer type
    fn shading(&self, diverge_num: f64, colour: Option<Color>) -> Color {
        let shade = escape_time(diverge_num, &self.pallete);
        // first layer can't be a shading layer so colour will be Some,
        // so unwrap will always succeed
        interpolate_colour(colour.unwrap(), BLACK, 1.0-shade.r)
    }

    /// calculate the colour for the shading3d layer type
    fn shading_3d(&self, t: f64, colour: Option<Color>) -> Color {
        // first layer can't be a shading layer so colour will be Some,
        // so unwrap will always succeed
        colour_3d(t, colour.unwrap())
    }

    /// calculate the colour for the colourorbittrap layer type
    fn orbit_trap_colour(&self, trapped_i: f64) -> Color {
        escape_time(trapped_i, &self.pallete)
    }

    fn orbit_trap_shading(&self, trapped_i: f64, colour: Option<Color>) -> Color {
        let shade = escape_time(trapped_i, &self.pallete);
        // first layer can't be a shading layer so colour will be Some,
        // so unwrap will always succeed
        interpolate_colour(colour.unwrap(), BLACK, 1.0-shade.r)
    }

    /// takes the generated colour and adds it to the current colour 
    /// taking into account the layer's strength
    fn final_colour(&self, colour: Option<Color>, this_colour: Color) -> Option<Color> {
        Some(match colour {
            Some(c) => interpolate_colour(c, this_colour, self.strength),
            None => this_colour
        })
    }

    /// determine the new colour for the pixel, using the implementor's output
    fn colour_implementors(&self, colour: Option<Color>, output: f64, in_set: bool) -> Option<Color> {
        if !self.layer_range.layer_applies(in_set) {
            return colour
        }

        let this_colour = match self.layer_type {
            LayerType::Colour => {self.colour(output)},
            LayerType::Shading => {self.shading(output, colour)},
            LayerType::ColourOrbitTrap(_) => {self.orbit_trap_colour(output)},
            LayerType::ShadingOrbitTrap(_) => {self.orbit_trap_shading(output, colour)},
            LayerType::Shading3D => {self.shading_3d(output, colour)}
        };

        self.final_colour(colour, this_colour)
    }
}