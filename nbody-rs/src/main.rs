#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,

    // @Temporary:
    unused_imports,
    dead_code,
)]


use std::mem;
use std::arch::x86_64::*;
use std::f64::consts::PI;


#[repr(C)]
struct body {
    position: [f64; 3],
    velocity: [f64; 3],
    mass: f64,
}


const SOLAR_MASS: f64 = 4.0 * PI * PI;
const DAYS_PER_YEAR: f64 = 356.24;
const BODIES_COUNT: usize = 5;


static mut solar_Bodies: [body; BODIES_COUNT] = [
    /* Sun */ body {
        mass: SOLAR_MASS,
        position: [0.0; 3],
        velocity: [0.0; 3],
    },

    /* Jupiter */ body {
        position: [
             4.84143144246472090e+00,
            -1.16032004402742839e+00,
            -1.03622044471123109e-01
        ],
        velocity: [
             1.66007664274403694e-03 * DAYS_PER_YEAR,
             7.69901118419740425e-03 * DAYS_PER_YEAR,
            -6.90460016972063023e-05 * DAYS_PER_YEAR
        ],
        mass: 9.54791938424326609e-04 * SOLAR_MASS
    },

    /* Saturn */ body {
        position: [
             8.34336671824457987e+00,
             4.12479856412430479e+00,
            -4.03523417114321381e-01
        ],
        velocity: [
            -2.76742510726862411e-03 * DAYS_PER_YEAR,
             4.99852801234917238e-03 * DAYS_PER_YEAR,
             2.30417297573763929e-05 * DAYS_PER_YEAR
        ],
        mass: 2.85885980666130812e-04 * SOLAR_MASS
    },

    /* Uranus */ body {
        position: [
             1.28943695621391310e+01,
            -1.51111514016986312e+01,
            -2.23307578892655734e-01
        ],
        velocity: [
             2.96460137564761618e-03 * DAYS_PER_YEAR,
             2.37847173959480950e-03 * DAYS_PER_YEAR,
            -2.96589568540237556e-05 * DAYS_PER_YEAR
        ],
        mass: 4.36624404335156298e-05 * SOLAR_MASS
    },

    /* Neptune */ body {
        position: [
             1.53796971148509165e+01,
            -2.59193146099879641e+01,
             1.79258772950371181e-01
        ],
        velocity: [
             2.68067772490389322e-03 * DAYS_PER_YEAR,
             1.62824170038242295e-03 * DAYS_PER_YEAR,
            -9.51592254519715870e-05 * DAYS_PER_YEAR
        ],
        mass: 5.15138902046611451e-05 * SOLAR_MASS
    },
];


fn main() {
    println!("Hello, world!");
}
