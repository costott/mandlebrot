// © 2023 costott. All rights reserved. 
// This code is provided for viewing purposes only. Copying, reproduction, 
// or distribution of this code, in whole or in part, in any form or by any 
// means, is strictly prohibited without prior written permission from the 
// copyright owner.

/// dead code that used to be used for specific divergence, now replaced by implementors

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
fn diverges_orbit_trap(c: ComplexType, max_iterations: u32, trap: &OrbitTrapType) -> f64 {
    let mut min_trap_distance2 = match trap {
        OrbitTrapType::Point(point) => point.greatest_distance2(),
        OrbitTrapType::Cross(cross) => cross.greatest_distance2(),
        OrbitTrapType::Circle(circle) => circle.greatest_distance2()
    };
    let divisor = min_trap_distance2.sqrt() / max_iterations as f64;
    let mut z = c.clone();

    for _ in 0..max_iterations {
        let z_trap_distance2 = match trap {
            OrbitTrapType::Point(point) => point.distance2(z.clone()),
            OrbitTrapType::Cross(cross) => cross.distance2(z.clone()),
            OrbitTrapType::Circle(circle) => circle.distance2(z.clone())
        };
        if z_trap_distance2 < min_trap_distance2 {
            min_trap_distance2 = z_trap_distance2;
        }
        if z.abs_squared() > BAILOUT_ORBIT_TRAP {
            // convert min trap distance as if working with max iterations
            return min_trap_distance2.sqrt() / divisor;
        }  
        z = z.square() + c.clone();
    }
    
    0.0
}

/// orbit trap colouring, returning t and the trapped i
fn diverges_orbit_trap_3d(c: ComplexType, max_iterations: u32, trap: &OrbitTrapType) -> (f64, f64) {
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
    let mut z = c.clone();
    let dc = ComplexType::same_type(1., 0., c.clone());
    let mut der = dc.clone();
    
    for _ in 0..max_iterations {
        let z_trap_distance2 = match trap {
            OrbitTrapType::Point(point) => point.distance2(z.clone()),
            OrbitTrapType::Cross(cross) => cross.distance2(z.clone()),
            OrbitTrapType::Circle(circle) => circle.distance2(z.clone())
        };
        if z_trap_distance2 < min_trap_distance2 {
            min_trap_distance2 = z_trap_distance2;
        }
        if z.abs_squared() > BAILOUT_ORBIT_TRAP {
            let mut u = z / der;
            u = u.clone() / f64::sqrt(u.abs_squared());
            let mut t = u.real_f64()*v.real + u.im_f64()*v.im + H2;
            t = t/(1.+H2);
            if t < 0. {t = 0.}; 

            // convert min trap distance as if working with max iterations
            // min_trap_distance2.sqrt() / divisor
            let trapped_i =  min_trap_distance2.sqrt() / divisor;
            return (t, trapped_i)
        }  
        der = der * (z.clone() * 2.) + dc.clone(); // brackets not needed but just to make more sense
        z = z.square() + c.clone();
    }
    (0.0, 0.0)
}

/// orbit trap colouring, returning t, trapped i, and smooth i
fn diverges_orbit_trap_3d_coloured(c: ComplexType, max_iterations: u32, trap: &OrbitTrapType) -> (f64, f64, f64) {
    let v = Complex::new(
        f64::cos(ANGLE * (PI / 180.)),
        f64::sin(ANGLE * (PI / 180.))
    );
    let mut min_trap_distance2 = trap.greatest_distance2();
    let divisor = min_trap_distance2.sqrt() / max_iterations as f64;
    let mut z = c.clone();
    let dc = ComplexType::same_type(1., 0., c.clone());
    let mut der = dc.clone();
    
    for i in 0..max_iterations {
        let z_trap_distance2 = trap.distance2(z.clone());
        if z_trap_distance2 < min_trap_distance2 {
            min_trap_distance2 = z_trap_distance2;
        }
        if z.abs_squared() > BAILOUT_ORBIT_TRAP {
            let mut u = z.clone() / der;
            u = u.clone() / f64::sqrt(u.abs_squared());
            let mut t = u.real_f64()*v.real + u.im_f64()*v.im + H2;
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
        der = der * (z.clone() * 2.) + dc.clone(); // brackets not needed but just to make more sense
        z = z.square() + c.clone();
    }
    (0.0, 0.0, 0.0)
}

/// orbit trap colouring, returning smooth i and the trapped_i for 1 trap
/// 
/// when only 1 trap use this instead of the function for multiple traps as this is more efficient
fn diverges_orbit_trap_coloured(c: ComplexType, max_iterations: u32, trap: OrbitTrapType) -> (f64, f64) {
    let mut min_trap_distance2 = trap.greatest_distance2();
    let divisor = min_trap_distance2.sqrt() / max_iterations as f64;
    let mut z = c.clone();

    for i in 0..max_iterations {
        let z_trap_distance2 = trap.distance2(z.clone());
        if z_trap_distance2 < min_trap_distance2 {
            min_trap_distance2 = z_trap_distance2;
        }
        if z.abs_squared() > BAILOUT_ORBIT_TRAP {
            // convert min trap distance as if working with max iterations
            let trapped_i =  min_trap_distance2.sqrt() / divisor;

            let log_zmod = f64::log2(z.abs_squared()) / 2.0;
            let nu = f64::log2(log_zmod);
            let smooth_iteration = i as f64 + 1.0 - nu;

            return (smooth_iteration, trapped_i)
        }  
        z = z.square() + c.clone();
    }
    (0.0, 0.0)
}

/// orbit trap colouring, returning smooth i and the trapped_i of all the given traps, in order
fn diverges_orbit_traps_coloured(c: ComplexType, max_iterations: u32, traps: &Vec<OrbitTrapType>) -> (f64, Vec<f64>) {
    let mut min_distance2s = Vec::with_capacity(traps.len());
    for trap in traps {
        min_distance2s.push(trap.greatest_distance2());
    }
    let mut divisors = Vec::with_capacity(traps.len());
    for i in 0..traps.len() {
        divisors.push(min_distance2s[i].sqrt() / max_iterations as f64);
    }
    let mut z = c.clone();
    
    for i in 0..max_iterations {
        for (i, trap) in traps.iter().enumerate() {
            let z_trap_distance2 = trap.distance2(z.clone());
            if z_trap_distance2 < min_distance2s[i] {
                min_distance2s[i] = z_trap_distance2;
            }
        }
        if z.abs_squared() > BAILOUT_ORBIT_TRAP {
            // convert min trap distance as if working with max iterations
            // min_trap_distance2.sqrt() / divisor
            let mut trapped_is = Vec::with_capacity(traps.len());
            for i in 0..traps.len() {
                trapped_is.push(min_distance2s[i].sqrt() / divisors[i]);
            }
            // let trapped_i =  min_trap_distance2.sqrt() / divisor;

            let log_zmod = f64::log2(z.abs_squared()) / 2.0;
            let nu = f64::log2(log_zmod);
            let smooth_iteration = i as f64 + 1.0 - nu;

            return  (smooth_iteration, trapped_is)
        }  
        z = z.square() + c.clone();
    }
    let mut trapped_is = Vec::with_capacity(traps.len());
    for _ in 0..traps.len() {
        trapped_is.push(0.0);
    }
    (0.0, trapped_is)
}

/// orbit trap colouring, returning smooth i, t, and the trapped_i of all the given traps, in order
fn diverges_orbit_traps_3d_coloured(c: ComplexType, max_iterations: u32, traps: &Vec<OrbitTrapType>) -> (f64, f64, Vec<f64>) {
    let v = Complex::new(
        f64::cos(ANGLE * (PI / 180.)),
        f64::sin(ANGLE * (PI / 180.))
    );
    let mut min_distance2s = Vec::with_capacity(traps.len());
    for trap in traps {
        min_distance2s.push(trap.greatest_distance2());
    }
    let mut divisors = Vec::with_capacity(traps.len());
    for i in 0..traps.len() {
        divisors.push(min_distance2s[i].sqrt() / max_iterations as f64);
    }
    let mut z = c.clone();
    let dc = ComplexType::same_type(1., 0., c.clone());
    let mut der = dc.clone();
    
    for i in 0..max_iterations {
        for (i, trap) in traps.iter().enumerate() {
            let z_trap_distance2 = trap.distance2(z.clone());
            if z_trap_distance2 < min_distance2s[i] {
                min_distance2s[i] = z_trap_distance2;
            }
        }
        if z.abs_squared() > BAILOUT_ORBIT_TRAP {
            let mut u = z.clone() / der;
            u = u.clone() / f64::sqrt(u.abs_squared());
            let mut t = u.real_f64()*v.real + u.im_f64()*v.im + H2;
            t = t/(1.+H2);
            if t < 0. {t = 0.}; 

            // convert min trap distance as if working with max iterations
            // min_trap_distance2.sqrt() / divisor
            let mut trapped_is = Vec::with_capacity(traps.len());
            for i in 0..traps.len() {
                trapped_is.push(min_distance2s[i].sqrt() / divisors[i]);
            }
            // let trapped_i =  min_trap_distance2.sqrt() / divisor;

            let log_zmod = f64::log2(z.abs_squared()) / 2.0;
            let nu = f64::log2(log_zmod);
            let smooth_iteration = i as f64 + 1.0 - nu;

            return  (smooth_iteration, t, trapped_is)
        }  
        der = der * (z.clone() * 2.) + dc.clone(); // brackets not needed but just to make more sense
        z = z.square() + c.clone();
    }
    let mut trapped_is = Vec::with_capacity(traps.len());
    for _ in 0..traps.len() {
        trapped_is.push(0.0);
    }
    (0.0, 0.0, trapped_is)
}