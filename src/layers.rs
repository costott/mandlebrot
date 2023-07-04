// © 2023 costott. All rights reserved. 
// This code is provided for viewing purposes only. Copying, reproduction, 
// or distribution of this code, in whole or in part, in any form or by any 
// means, is strictly prohibited without prior written permission from the 
// copyright owner.

use macroquad::prelude::*;

use std::ops::Range;

use crate::palettes::Palette;

use super::{*, menu::DropDownType};

/// analyse the given complex number, letting the implementors
/// calculate their outputs
/// 
/// # For double precision complex numbers
/// 
/// # Returns
/// returns if the point is in the set or not
fn diverges_implementors_double(c: Complex, max_iterations: u32, implementations: &mut Vec<LayerImplementation>) -> bool {
    let mut z = c;
    for im in implementations.iter_mut() {
        im.before(max_iterations);
    }

    for i in 0..max_iterations {        
        // if z.real.abs() > BAILOUT_ORBIT_TRAP || z.im.abs() > BAILOUT_ORBIT_TRAP { <- much faster but less accurate
        if z.abs_squared() > BAILOUT_ORBIT_TRAP {
            for im in implementations.iter_mut() {
                im.out_set_double(z, i);
            }
            return false;
        }

        for im in implementations.iter_mut() {
            im.during_double(z, i);
        }

        z = z.square() + c;
    }

    for im in implementations.iter_mut() {
        im.in_set_double(z);
    }
    return true;
}

/// analyse the given complex number, letting the implementors
/// calculate their outputs
/// 
/// # For arbitrary precision complex numbers
/// 
/// # Returns
/// returns if the point is in the set or not
fn diverges_implementors_big(c: BigComplex, max_iterations: u32, implementations: &mut Vec<LayerImplementation>) -> bool {
    let mut z = c.clone();
    for im in implementations.iter_mut() {
        im.before(max_iterations);
    }

    for i in 0..max_iterations {        
        // if z.real.abs() > BAILOUT_ORBIT_TRAP || z.im.abs() > BAILOUT_ORBIT_TRAP { <- much faster but less accurate
        if z.abs_squared() > BAILOUT_ORBIT_TRAP {
            for im in implementations.iter_mut() {
                im.out_set_big(&z, i);
            }
            return false;
        }

        for im in implementations.iter_mut() {
            im.during_big(&z, i);
        }

        z = &z.square() + &c;
    }

    for im in implementations.iter_mut() {
        im.in_set_big(&z);
    }
    true
}

/// analyse the given complex number, letting the implementors
/// calculate their outputs
/// 
/// **using mandelbrot perturbation theory**
/// 
/// # Returns
/// returns if the point is in the set or not
fn diverges_implementors_big_perturbation(dc: Complex, ref_z: &Vec<Complex>, max_ref_iteration: usize, max_iterations: u32, implementations: &mut Vec<LayerImplementation>) -> bool {
    let mut dz = Complex::new(0., 0.);
    // https://fractalforums.org/fractal-mathematics-and-new-theories/28/another-solution-to-perturbation-glitches/4360/msg29835#msg29835
    let mut ref_iteration = 0;

    for im in implementations.iter_mut() {
        im.before(max_iterations);
    }

    for i in 0..max_iterations {
        dz =  ref_z[ref_iteration] * dz * 2. + dz.square() + dc;
        ref_iteration += 1;

        let z2 = ref_z[ref_iteration] + dz;

        if z2.abs_squared() > BAILOUT_ORBIT_TRAP {
            for im in implementations.iter_mut() {
                im.out_set_double(z2, i);
            }
            return false;
        }
        if z2.abs_squared() < dz.abs_squared() || ref_iteration == max_ref_iteration {
            dz = z2;
            ref_iteration = 0;
        }

        for im in implementations.iter_mut() {
            im.during_double(z2, i);
        }
    }

    for im in implementations.iter_mut() {
        im.in_set_double(ref_z[ref_iteration] + dz);
    }

    true
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
    pub fn get_string(&self) -> String{
        String::from(match self {
            LayerType::Colour => "Colour",
            LayerType::ColourOrbitTrap(_) => "Colour (orbit trap)",
            LayerType::Shading => "Shading",
            LayerType::Shading3D => "Shading 3D",
            LayerType::ShadingOrbitTrap(_) => "Shading (orbit trap)"
        })
    }

    /// returns whether the layer is a shading layer
    pub fn shading_layer(&self) -> bool {
        match self {
            LayerType::Shading | LayerType::Shading3D | &LayerType::ShadingOrbitTrap(_) => {
                true
            },
            _ => false
        }
    }

    fn get_default_orbit_trap() -> OrbitTrapType {
        OrbitTrapType::Point(OrbitTrapPoint::default())
    }

    pub fn is_orbit_trap(&self) -> bool {
        match self  {
            LayerType::ColourOrbitTrap(_) | &LayerType::ShadingOrbitTrap(_) => true,
            _ => false
        }
    }

    pub fn get_orbit_trap(&mut self) -> Result<&mut OrbitTrapType, &str> {
        match self {
            LayerType::ColourOrbitTrap(trap) | LayerType::ShadingOrbitTrap(trap) => Ok(trap),
            _ => Err("not a trap")
        }
    }
}
impl DropDownType<LayerType> for LayerType {
    fn get_variants() -> Vec<LayerType> {
        vec![
            LayerType::Colour, 
            LayerType::ColourOrbitTrap(LayerType::get_default_orbit_trap()),
            LayerType::Shading,
            LayerType::Shading3D,
            LayerType::ShadingOrbitTrap(LayerType::get_default_orbit_trap())
        ]
    }

    fn get_string(&self) -> String {
        self.get_string()
    }
}
impl PartialEq for LayerType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            LayerType::Colour => match other  {
                LayerType::Colour => true,
                _ => false
            },
            LayerType::ColourOrbitTrap(_) => match other {
                LayerType::ColourOrbitTrap(_) => true,
                _ => false
            },
            LayerType::Shading => match other {
                LayerType::Shading => true,
                _ => false
            },
            LayerType::Shading3D => match other {
                LayerType::Shading3D => true,
                _ => false
            },
            LayerType::ShadingOrbitTrap(_) => match other {
                LayerType::ShadingOrbitTrap(_) => true,
                _ => false
            }
        }
    }
}

// this should be made for generation not by the user
/// Controls the behaviour of layer implementors when analysing a point
trait LayerImplementor {
    /// what needs to happen before iterations start for a given pixel
    fn before(&mut self, max_iterations: u32);

    /// what needs to happen each iteration, before the next z is calculated
    /// for double precision
    fn during_double(&mut self, z: Complex, i: u32);
    /// what needs to happen each iteration, before the next z is calculated
    /// for arbitrary precision
    fn during_big(&mut self, z: &BigComplex, i: u32);

    /// what needs to happen if the point is outside the set
    /// for double precision
    fn out_set_double(&mut self, z: Complex, i: u32);
    /// what needs to happen if the point is outside the set
    /// for arbitrary precision
    fn out_set_big(&mut self, z: &BigComplex, i: u32);

    /// what needs to happen if the point is inside the set
    /// for double precision
    fn in_set_double(&mut self, z: Complex);
    /// what needs to happen if the point is inside the set
    /// for arbitrary precision
    fn in_set_big(&mut self, z: &BigComplex);

    /// output the generated value
    fn get_output(&self) -> f64;
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
    fn before(&mut self, max_iterations: u32) {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.before(max_iterations),
            LayerImplementation::OrbitTrapImplementor(im) => im.before(max_iterations),
            LayerImplementation::Shading3DImplementor(im) => im.before(max_iterations)
        }
    }

    fn during_double(&mut self, z: Complex, i: u32) {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.during_double(z, i),
            LayerImplementation::OrbitTrapImplementor(im) => im.during_double(z, i),
            LayerImplementation::Shading3DImplementor(im) => im.during_double(z, i)
        }
    }

    fn during_big(&mut self, z: &BigComplex, i: u32) {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.during_big(z, i),
            LayerImplementation::OrbitTrapImplementor(im) => im.during_big(z, i),
            LayerImplementation::Shading3DImplementor(im) => im.during_big(z, i)
        }
    }

    fn out_set_double(&mut self, z: Complex, i: u32) {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.out_set_double(z, i),
            LayerImplementation::OrbitTrapImplementor(im) => im.out_set_double(z, i),
            LayerImplementation::Shading3DImplementor(im) => im.out_set_double(z, i)
        }
    }

    fn out_set_big(&mut self, z: &BigComplex, i: u32) {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.out_set_big(z, i),
            LayerImplementation::OrbitTrapImplementor(im) => im.out_set_big(z, i),
            LayerImplementation::Shading3DImplementor(im) => im.out_set_big(z, i)
        }
    }

    fn in_set_double(&mut self, z: Complex) {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.in_set_double(z),
            LayerImplementation::OrbitTrapImplementor(im) => im.in_set_double(z),
            LayerImplementation::Shading3DImplementor(im) => im.in_set_double(z)
        }
    }

    fn in_set_big(&mut self, z: &BigComplex) {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.in_set_big(z),
            LayerImplementation::OrbitTrapImplementor(im) => im.in_set_big(z),
            LayerImplementation::Shading3DImplementor(im) => im.in_set_big(z)
        }
    }

    fn get_output(&self) -> f64 {
        match self {
            LayerImplementation::ColourImplemetor(im) => im.get_output(),
            LayerImplementation::OrbitTrapImplementor(im) => im.get_output(),
            LayerImplementation::Shading3DImplementor(im) => im.get_output()
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

    fn out_set(&mut self, abs2_z: f64, i: u32) {
        let log_zmod = f64::log2(abs2_z) / 2.0;
        let nu = f64::log2(log_zmod);
        let smooth_iteration = i as f64 + 1.0 - nu;
        self.output = smooth_iteration;
    }
}
impl LayerImplementor for ColourImplemetor {
    fn before(&mut self, _max_iterations: u32) {}

    fn during_double(&mut self, _z: Complex, _i: u32) {}
    fn during_big(&mut self, _z: &BigComplex, _i: u32) {}

    fn out_set_double(&mut self, z: Complex, i: u32) {
        self.out_set(z.abs_squared(), i);
    }
    fn out_set_big(&mut self, z: &BigComplex, i: u32) {
        self.out_set(z.abs_squared(), i);
    }

    fn in_set_double(&mut self, _z: Complex) {
        self.output = 0.0;
    }
    fn in_set_big(&mut self, _z: &BigComplex) {
        self.output = 0.0;
    }

    fn get_output(&self) -> f64 {
        self.output
    }
}

#[derive(Clone)]
/// orbit trapped algorithm looking at the minimum distance between an orbit and a trap,
/// calculating a trapped index to be used in the palette
struct OrbitTrapImplementor {
    output: f64,
    min_distance2: f64,
    divisor: f64,
    trap: OrbitTrapType,
    /// 'vector' of closest point to the trap
    closest_to_trap: Complex,
    closest_to_trap_big: BigComplex
}
impl OrbitTrapImplementor {
    fn new(trap: OrbitTrapType) -> OrbitTrapImplementor {
        OrbitTrapImplementor { 
            output: 0.0, 
            min_distance2: trap.greatest_distance2(),
            divisor: 0.0, 
            trap,
            closest_to_trap: Complex::new(0.0, 0.0),
            closest_to_trap_big: BigComplex::from_f64s(0.0, 0.0)
        }
    }

    fn generate_output_double(&self) -> f64 {
        let output = match self.trap.get_analysis() {
            OrbitTrapAnalysis::Distance => self.min_distance2.sqrt(),
            OrbitTrapAnalysis::Real => self.closest_to_trap.real_f64().abs(),
            OrbitTrapAnalysis::Imaginary => self.closest_to_trap.im_f64().abs(),
            OrbitTrapAnalysis::Angle => PI + self.closest_to_trap.arg()
        } / self.divisor;
        output
    }
    fn generate_output_big(&self) -> f64 {
        let output = match self.trap.get_analysis() {
            OrbitTrapAnalysis::Distance => self.min_distance2.sqrt(),
            OrbitTrapAnalysis::Real => self.closest_to_trap_big.real_f64().abs(),
            OrbitTrapAnalysis::Imaginary => self.closest_to_trap_big.im_f64().abs(),
            OrbitTrapAnalysis::Angle => PI + self.closest_to_trap_big.arg()
        } / self.divisor;
        output
    }
}
impl LayerImplementor for OrbitTrapImplementor {
    fn before(&mut self, max_iterations: u32) {
        self.divisor = self.min_distance2.sqrt() / max_iterations as f64;
    }

    fn during_double(&mut self, z: Complex, _i: u32) {
        // if i == 0 {return}
        let z_trap_distance2 = self.trap.distance2_double(z);
        if z_trap_distance2 < self.min_distance2 {
            self.min_distance2 = z_trap_distance2;
            self.closest_to_trap = self.trap.vector_double(z);
        }
    }
    fn during_big(&mut self, z: &BigComplex, _i: u32) {
        // if i == 0 {return}
        let z_trap_distance2 = self.trap.distance2_big(z);
        if z_trap_distance2 < self.min_distance2 {
            self.min_distance2 = z_trap_distance2;
            self.closest_to_trap_big = self.trap.vector_big(z);
        }
    }

    fn out_set_double(&mut self, _z: Complex, _i: u32) {
        self.output = self.generate_output_double();
    }
    fn out_set_big(&mut self, _z: &BigComplex, _i: u32) {
        self.output = self.generate_output_big();
    }

    fn in_set_double(&mut self, _z: Complex) {
        self.output = self.generate_output_double();
    }
    fn in_set_big(&mut self, _z: &BigComplex) {
        self.output = self.generate_output_big();
    }

    fn get_output(&self) -> f64 {
        self.output
    }
}

#[derive(Clone)]
/// 3d algorithm to shade the set to give height.
/// Theory from: https://www.math.univ-toulouse.fr/~cheritat/wiki-draw/index.php/Mandelbrot_set#Normal_map_effect,
/// calculating a t value that represents darkness/brightness
struct Shading3DImplementor {
    output: f64,
    v: Complex,
    v_big: BigComplex,
    der: Complex,
    der_big: BigComplex,
    dc: Complex,
    dc_big: BigComplex
}
impl Shading3DImplementor {
    fn new() -> Shading3DImplementor {
        Shading3DImplementor { 
            output: 0.0, 
            v: Complex::new(
                f64::cos(ANGLE * (PI / 180.)),
                f64::sin(ANGLE * (PI / 180.))
            ),
            v_big: BigComplex::from_f64s(
                f64::cos(ANGLE * (PI / 180.)),
                f64::sin(ANGLE * (PI / 180.))
            ),
            der: Complex::new(1., 0.), 
            der_big: BigComplex::from_f64s(1., 0.),
            dc: Complex::new(1., 0.),
            dc_big: BigComplex::from_f64s(1., 0.)
        }
    }

    fn generate_output_double(&self, z: Complex) -> f64 {
        let mut u = z / self.der;
        u = &u / f64::sqrt(u.abs_squared());
        let t = u.real*self.v.real + u.im*self.v.im;
        let mut t = t+ H2;
        t = t/(1.+H2);
        t = f64::max(0.1, t);
        t
    }

    fn generate_output_big(&self, z: &BigComplex) -> f64 {
        let mut u = z / &self.der_big;
        u = &u / f64::sqrt(u.abs_squared());
        let t = u.real_f64()*self.v_big.real_f64() + u.im_f64()*self.v_big.im_f64();
        let mut t = t+ H2;
        t = t/(1.+H2);
        t = f64::max(0.1, t);
        t
    }
}
impl LayerImplementor for Shading3DImplementor {
    fn before(&mut self, _max_iterations: u32) {}

    fn during_double(&mut self, z: Complex, _i: u32) {
        self.der = self.der * (z * 2.) + self.dc;
    }
    fn during_big(&mut self, z: &BigComplex, _i: u32) {
        self.der_big = &self.der_big * (z * 2.) + &self.dc_big;
    }

    fn out_set_double(&mut self, z: Complex, _i: u32) {
        self.output = self.generate_output_double(z);
    }
    fn out_set_big(&mut self, z: &BigComplex, _i: u32) {
        self.output = self.generate_output_big(z);
    }

    fn in_set_double(&mut self, z: Complex) {
        self.output = self.generate_output_double(z);
    }
    fn in_set_big(&mut self, z: &BigComplex) {
        self.output = self.generate_output_big(z);
    }

    fn get_output(&self) -> f64 {
        self.output
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
        let valid = Layers::valid_layers(&layers);
        if let Err(e) = valid {
            panic!("{e}");
        }

        let mut layers = layers;
        Layers::place_constraints(&mut layers);
        for (i, layer) in layers.iter_mut().enumerate() {
            layer.name = format!("Layer {}", i + 1);
        }

        let (implementors, implementor_map) = make_implementors(&layers);
        
        Layers { layers, implementors, implementor_map, arb_precision: false }
    }

    /// returns if the given layer configuration is allowed or not
    fn valid_layers(layers: &Vec<Layer>) -> Result<bool, &'static str> {
        // has to be multiple layers
        if layers.len() == 0 {
            return Err("there must be at least 1 layer");
        }

        // the first layer can't be a shading layer
        if match layers[0].layer_type {
            LayerType::Shading | LayerType::Shading3D |
            LayerType::ShadingOrbitTrap(_) => 1,
            _ => 0
        } == 1 {
            return Err("the first layer can't be a shading layer");
        }

        // shading layers have to shade colours
        let mut non_shade_in_set = false;
        let mut non_shade_out_set = false;
        for layer in layers.iter() {
            if layer.layer_type.shading_layer() { 
                if layer.layer_range.layer_covered(non_shade_in_set, non_shade_out_set) { continue }
                return Err("Shading layer exists that isn't covered by another layer");
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

        Ok(true)
    }

    fn clear_constraints(layers: &mut Vec<Layer>) {
        for layer in layers {
            layer.clear_contraints();
        }
    }

    fn get_colours_before_first_shade(layers: &Vec<Layer>, first_shade_i: usize, in_set: bool) -> (usize, usize) {
        let mut first_colour_i = 0;
        let mut colours_before_first_shade = 0;
        for i in 0..first_shade_i {
            let layer = &layers[i];
            if !layer.layer_type.shading_layer() && layer.layer_range.layer_applies(in_set) {
                colours_before_first_shade += 1;
                if first_colour_i == 0 {
                    first_colour_i = i;
                }
            }
        }

        (first_colour_i, colours_before_first_shade)
    }

    fn place_range_constraints(layers: &mut Vec<Layer>) {
        if layers.len() == 1 {
            let layer = &mut layers[0];
            // ensure this layer exists in some form and can pick any state
            layer.set_range_constraint(match layer.layer_range {
                LayerRange::InSet => LayerRange::OutSet,
                LayerRange::OutSet => LayerRange::InSet,
                LayerRange::Both => LayerRange::OutSet
            });
            return;
        }

        let colour_in_set: usize = layers.iter().filter(|l| {
            !l.layer_type.shading_layer() && l.layer_range.layer_applies(true)
        }).collect::<Vec<&Layer>>().len();
        let colour_out_set: usize = layers.iter().filter(|l| {
            !l.layer_type.shading_layer() && l.layer_range.layer_applies(false)
        }).collect::<Vec<&Layer>>().len();
        // force shading layers to stay in their ranges if no colour layer in other range
        for layer in layers.iter_mut() {
            if colour_in_set == 0 && layer.layer_type.shading_layer() && layer.layer_range.layer_applies(false) {
                layer.set_range_constraint(LayerRange::OutSet);
            }
            if colour_out_set == 0 && layer.layer_type.shading_layer() && layer.layer_range.layer_applies(true) {
                layer.set_range_constraint(LayerRange::InSet);
            }
        }

        let mut first_shade_in_set: Option<usize> = None;
        let mut first_shade_out_set: Option<usize> = None;
        for (i, layer) in layers.iter().enumerate() {
            if layer.layer_type.shading_layer() && layer.layer_range.layer_applies(true) && first_shade_in_set.is_none() {
                first_shade_in_set = Some(i);
            }
            if layer.layer_type.shading_layer() && layer.layer_range.layer_applies(false) && first_shade_out_set.is_none() {
                first_shade_out_set = Some(i);
            }
        }

        if let Some(first_shade_i) = first_shade_in_set {
            let (first_colour_i, colours_before_first_shade) = 
                Layers::get_colours_before_first_shade(layers, first_shade_i, true);
            if colours_before_first_shade == 1 {
                layers[first_colour_i].set_range_constraint(LayerRange::InSet);
            }
        }   
        if let Some(first_shade_i) = first_shade_out_set {
            let (first_colour_i, colours_before_first_shade) = 
                Layers::get_colours_before_first_shade(layers, first_shade_i, false);
            if colours_before_first_shade == 1 {
                layers[first_colour_i].set_range_constraint(LayerRange::OutSet);
            }
        }
    }

    fn place_position_constraints(layers: &mut Vec<Layer>) {
        let end = layers.len()+1;

        let mut first_shade_in_set: Option<usize> = None;
        let mut first_colour_in_set: Option<usize> = None;
        let mut first_shade_out_set: Option<usize> = None;
        let mut first_colour_out_set: Option<usize> = None;
        for (i, layer) in layers.iter_mut().enumerate() {
            // find first colour layers
            if !layer.layer_type.shading_layer() && layer.layer_range.layer_applies(true) && first_colour_in_set.is_none() {
                first_colour_in_set = Some(i);
            }
            if !layer.layer_type.shading_layer() && layer.layer_range.layer_applies(false) && first_colour_out_set.is_none() {
                first_colour_out_set = Some(i);
            }

            // set shading layers' constraints to never be before the first colour
            if layer.layer_type.shading_layer() && layer.layer_range.layer_applies(true){
                layer.set_position_constraint(first_colour_in_set.unwrap()+1..end);
                if first_shade_in_set.is_none() {
                    first_shade_in_set = Some(i);
                }
            }
            if layer.layer_type.shading_layer() && layer.layer_range.layer_applies(false){
                layer.set_position_constraint(first_colour_out_set.unwrap()+1..end);
                if first_shade_out_set.is_none() {
                    first_shade_out_set = Some(i);
                }
            }    
        }

        // unwrap will always succeed as for a shading layer to apply 
        // a colour layer has to also be applied before
        if let Some(shade_i) = first_shade_in_set {
            layers.iter_mut().filter(|l| {
                !l.layer_type.shading_layer() && l.layer_range.layer_applies(true)
            }).collect::<Vec<&mut Layer>>().first_mut().unwrap().set_position_constraint(0..shade_i+1);
        }
        if let Some(shade_i) = first_shade_out_set {
            layers.iter_mut().filter(|l| {
                !l.layer_type.shading_layer() && l.layer_range.layer_applies(false)
            }).collect::<Vec<&mut Layer>>().first_mut().unwrap().set_position_constraint(0..shade_i+1);
        }
    }

    pub fn place_constraints(layers: &mut Vec<Layer>) {
        Layers::clear_constraints(layers);

        Layers::place_range_constraints(layers);
        Layers::place_position_constraints(layers);
    }

    pub fn update_implementors(&mut self) {
        (self.implementors, self.implementor_map) = make_implementors(&self.layers);
    }

    pub fn add_layer(&mut self, layer: &Layer) {
        let mut layer = layer.clone();
        layer.name = format!("Layer {}", self.layers.len()+1);
        self.layers.push(layer);

        Layers::place_constraints(&mut self.layers);
        self.update_implementors();
    }

    /// take the layer at the take_i from its position and insert it at the dest_i
    pub fn reorder_layer(&mut self, take_i: usize, dest_i: usize) {
        let to_insert = self.layers[take_i].clone();
        self.layers.remove(take_i);
        self.layers.insert(dest_i, to_insert);

        self.update_implementors();
    }

    pub fn delete_layer(&mut self, index: usize) {
        self.layers.remove(index);

        self.update_implementors();
    }

    pub fn change_layer_type(&mut self, index: usize, new_type: LayerType) {
        if !self.layers[index].can_change_type(&new_type) { return }
        let old_type = self.layers[index].layer_type.clone();
        self.layers[index].layer_type = new_type;
        if Layers::valid_layers(&self.layers).is_err() {
            self.layers[index].layer_type = old_type
        }

        self.update_implementors();
    }

    /// makes sure all the palettes for the layers
    /// are updated for the current max iterations
    pub fn generate_palettes(&mut self, max_iterations: f32) {
        for layer in self.layers.iter_mut() {
            layer.palette.generate_palette(max_iterations);
        }
    }

    /// get the colour for the given complex number after passing 
    /// through all the layers
    pub fn colour_pixel(&self, c: ComplexType, max_iterations: u32) -> Color {
        self.colour_pixel_implementors(c, max_iterations)
    }

    fn colour_pixel_implementors(&self, c: ComplexType, max_iterations: u32) -> Color {
        let mut implementors = self.implementors.clone();
        let in_set = match c {
            ComplexType::Double(c) => diverges_implementors_double(c, max_iterations, &mut implementors),
            ComplexType::Big(c) => diverges_implementors_big(c, max_iterations, &mut implementors)
        };

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

    pub fn colour_pixel_implementors_perturbed(&self, dc: Complex, ref_z: &Vec<Complex>, max_ref_iteration: usize, max_iterations: u32) -> Color {
        let mut implementors = self.implementors.clone();
        let in_set = diverges_implementors_big_perturbation(dc, ref_z, max_ref_iteration, max_iterations, &mut implementors);

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

#[derive(Clone, Copy, PartialEq, Debug)]
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
impl DropDownType<LayerRange> for LayerRange {
    fn get_variants() -> Vec<LayerRange> {
        vec![LayerRange::InSet, LayerRange::OutSet, LayerRange::Both]
    }

    fn get_string(&self) -> String {
        String::from(match self {
            LayerRange::InSet => "In Set",
            LayerRange::OutSet => "Out Set",
            LayerRange::Both => "Both"
        })
    }
}

/// A colouring layer for the mandelbrot set
#[derive(Clone)]
pub struct Layer {
    pub name: String,
    pub layer_type: LayerType,
    pub layer_range: LayerRange,
    /// stores the only ranges this layer is allowed
    range_constraints: Option<Vec<LayerRange>>,
    /// stores the maximum position the layer's allowed to be at
    position_constraint: Option<Range<usize>>,
    pub strength: f32,
    pub palette: Palette
}
impl Layer {
    /// # Params
    /// `layer_type`: the type of colouring it does
    /// 
    /// `strength`: how much of the current colour generated by the previous layers should it override
    /// **0.0 => None, 1.0 => All**
    pub fn new(
        layer_type: LayerType, 
        layer_range: LayerRange, 
        strength: f32, 
        palette: Palette
    ) -> Layer {
        Layer {
            layer_type, layer_range, strength, palette,
            range_constraints: None,
            position_constraint: None,
            name: String::from("Layer")
        }
    }

    fn set_range_constraint(&mut self, constraint: LayerRange) {
        match self.range_constraints {
            Some(ref mut v) => v.push(constraint),
            None => {
                self.range_constraints = Some(vec![constraint])
            }
        }
    }
    
    fn set_position_constraint(&mut self, constraint: Range<usize>) {
        self.position_constraint = Some(constraint);
    }

    fn clear_contraints(&mut self) {
        self.range_constraints = None;
        self.position_constraint = None;
    }   
    
    /// looks at its constraints to see if it could be the given range
    fn range_allowed(&self, range: LayerRange) -> bool {
        match self.range_constraints {
            None => true,
            Some(ref v) => {
                let mut in_set = false;
                let mut out_set = false;
                for layer in v {
                    if layer.layer_applies(true) {
                        in_set = true;
                    }
                    if layer.layer_applies(false) {
                        out_set = true;
                    }
                }

                match (in_set, out_set) {
                    (false, false) => true,
                    (true, false) => {
                        if self.layer_type.shading_layer() { range == LayerRange::InSet}
                        else {range.layer_applies(true)}
                    },
                    (false, true) => {
                        if self.layer_type.shading_layer() { range == LayerRange::InSet }
                        else {range.layer_applies(false)}
                    },
                    (true, true) => range == LayerRange::Both
                }
            }
        }
    }

    /// looks at its constraints to see if it could be in the given position
    pub fn position_allowed(&self, position: usize) -> bool {
        match &self.position_constraint {
            None => true,
            Some(pos) => pos.contains(&position)
        }
    }

    pub fn can_delete(&self) -> bool {
        return self.range_constraints.is_none() || self.layer_type.shading_layer()
    }

    pub fn can_change_type(&self, new_type: &LayerType) -> bool {
        if self.layer_type.shading_layer() { return true }

        if !new_type.shading_layer() { return true }

        self.range_constraints.is_none()
    }

    pub fn change_strength(&mut self, new: f32) -> bool {
        if self.strength == new { return false }
        self.strength = new;
        return true;
    }

    pub fn change_range(&mut self, new: LayerRange) {
        if self.range_allowed(new) {
            self.layer_range = new;
        }
    }

    /// calculate the colour for the Colour layer type
    fn colour(&self, diverge_num: f64) -> Color {
        escape_time(diverge_num, &self.palette.palette_cache)
    }

    /// calculate the colour for the Shading layer type
    fn shading(&self, diverge_num: f64, colour: Option<Color>) -> Color {
        let shade = escape_time(diverge_num, &self.palette.palette_cache);
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
        escape_time(trapped_i, &self.palette.palette_cache)
    }

    fn orbit_trap_shading(&self, trapped_i: f64, colour: Option<Color>) -> Color {
        let shade = escape_time(trapped_i, &self.palette.palette_cache);
        // first layer can't be a shading layer so colour will be Some,
        // so unwrap will always succeed
        interpolate_colour(colour.unwrap(), BLACK, 1.0-shade.r)
    }

    /// takes the generated colour and adds it to the current colour 
    /// taking into account the layer's strength
    fn final_colour(&self, colour: Option<Color>, this_colour: Color) -> Option<Color> {
        Some(match colour {
            Some(c) => interpolate_colour(c, this_colour, self.strength),
            None => interpolate_colour(BLACK, this_colour, self.strength)
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