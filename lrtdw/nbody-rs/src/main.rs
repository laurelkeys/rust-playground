// http://cliffle.com/p/dangerust/

#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
)]

use std::arch::x86_64::*;
use std::f64::consts::PI;

#[repr(C)]
struct body {
    position: [f64; 3],
    velocity: [f64; 3],
    mass: f64,
}

const SOLAR_MASS: f64 = 4.0 * PI * PI;
const DAYS_PER_YEAR: f64 = 365.24;
const BODIES_COUNT: usize = 5;

const STARTING_STATE: [body; BODIES_COUNT] = [
    /* Sun */ body {
        mass: SOLAR_MASS,
        position: [0.0; 3],
        velocity: [0.0; 3],
    },

    /* Jupiter */ body {
        position: [
             4.84143144246472090e+00,
            -1.16032004402742839e+00,
            -1.03622044471123109e-01,
        ],
        velocity: [
             1.66007664274403694e-03 * DAYS_PER_YEAR,
             7.69901118419740425e-03 * DAYS_PER_YEAR,
            -6.90460016972063023e-05 * DAYS_PER_YEAR,
        ],
        mass: 9.54791938424326609e-04 * SOLAR_MASS,
    },

    /* Saturn */ body {
        position: [
             8.34336671824457987e+00,
             4.12479856412430479e+00,
            -4.03523417114321381e-01,
        ],
        velocity: [
            -2.76742510726862411e-03 * DAYS_PER_YEAR,
             4.99852801234917238e-03 * DAYS_PER_YEAR,
             2.30417297573763929e-05 * DAYS_PER_YEAR,
        ],
        mass: 2.85885980666130812e-04 * SOLAR_MASS,
    },

    /* Uranus */ body {
        position: [
             1.28943695621391310e+01,
            -1.51111514016986312e+01,
            -2.23307578892655734e-01,
        ],
        velocity: [
             2.96460137564761618e-03 * DAYS_PER_YEAR,
             2.37847173959480950e-03 * DAYS_PER_YEAR,
            -2.96589568540237556e-05 * DAYS_PER_YEAR,
        ],
        mass: 4.36624404335156298e-05 * SOLAR_MASS,
    },

    /* Neptune */ body {
        position: [
             1.53796971148509165e+01,
            -2.59193146099879641e+01,
             1.79258772950371181e-01,
        ],
        velocity: [
             2.68067772490389322e-03 * DAYS_PER_YEAR,
             1.62824170038242295e-03 * DAYS_PER_YEAR,
            -9.51592254519715870e-05 * DAYS_PER_YEAR,
        ],
        mass: 5.15138902046611451e-05 * SOLAR_MASS,
    },
];

// Total number of different interactions between each body and every other body.
const INTERACTIONS_COUNT: usize = BODIES_COUNT * (BODIES_COUNT - 1) / 2;

// @Note: Some of the calculations will be computed two at a time by using x86 SSE
// instructions, so it will be useful to have a `ROUNDED_INTERACTIONS_COUNT` which
// is the smallest even number which is equal to or greater than `INTERACTIONS_COUNT`.
const ROUNDED_INTERACTIONS_COUNT: usize = INTERACTIONS_COUNT + INTERACTIONS_COUNT % 2;

// @Note: Both of these arrays are set to contain `ROUNDED_INTERACTIONS_COUNT`
// elements to simplify one of the following loops, and to also keep the
// second and third arrays in `position_Deltas` aligned properly.
#[derive(Copy, Clone)]
#[repr(C)]
union Interactions {
    scalars: [f64; ROUNDED_INTERACTIONS_COUNT],
    vectors: [__m128d; ROUNDED_INTERACTIONS_COUNT / 2],
}

impl Interactions {
    // @Safety: The in-memory representation of `f64` and `__m128d` is
    // compatible, so accesses to the union members is safe in any order.

    /// Returns a reference to the storage as `f64`s.
    pub fn as_scalars(&mut self) -> &mut [f64; ROUNDED_INTERACTIONS_COUNT] {
        unsafe { &mut self.scalars }
    }

    /// Returns a reference to the storage as `__m128d`s.
    pub fn as_vectors(&mut self) -> &mut [__m128d; ROUNDED_INTERACTIONS_COUNT / 2] {
        unsafe { &mut self.vectors }
    }
}

/// Advance all the bodies in the system by one timestep.
/// Calculate the interactions between all the bodies, update each body's
/// velocity based on those interactions, and update each body's position
/// by the distance it travels in a timestep at it's updated velocity.
#[cfg(target_feature = "sse2")]
fn advance(
    bodies: &mut [body; BODIES_COUNT],
    position_Deltas: &mut [Interactions; 3],
    magnitudes: &mut Interactions,
) {
    // @Safety: This code is only compiled for processors that support SSE2,
    // so the SIMD operations used here are safe.

    // Calculate the position_Deltas between the bodies for each interaction.
    {
        let mut k = 0;
        for i in 0..(BODIES_COUNT - 1) {
            for j in (i + 1)..BODIES_COUNT {
                for m in 0..3 {
                    position_Deltas[m].as_scalars()[k] = bodies[i].position[m] - bodies[j].position[m];
                }

                k += 1;
            }
        }
    }

    // Calculate the magnitudes of force between the bodies for each interaction.
    // @Note: This loop processes two interactions at a time,
    // which is why `ROUNDED_INTERACTIONS_COUNT / 2` iterations are done.
    for i in 0..(ROUNDED_INTERACTIONS_COUNT / 2) {
        // Load `position_Deltas` of two bodies into `position_Delta`.
        let mut position_Delta = [unsafe { _mm_setzero_pd() }; 3];
        for m in 0..3 {
            position_Delta[m] = position_Deltas[m].as_vectors()[i];
        }

        // @Note: Doing square roots using double precision floating point math can be
        // quite time consuming, so SSE's much faster single precision reciprocal square root
        // approximation instruction is used as a starting point instead.
        let distance_Squared: __m128d = unsafe {
            _mm_add_pd(                                                 // = ((position_Delta[0] * position_Delta[0])
                _mm_add_pd(                                             //     + (position_Delta[1] * position_Delta[1]))
                    _mm_mul_pd(position_Delta[0], position_Delta[0]),   //     + (position_Delta[2] * position_Delta[2])
                    _mm_mul_pd(position_Delta[1], position_Delta[1])),  //
                _mm_mul_pd(position_Delta[2], position_Delta[2]))       //
        };

        // @Note: Since the precision isn't quite sufficient to get acceptable results,
        // so two iterations of the Newton–Raphson method are done to improve it further.
        let mut distance_Reciprocal: __m128d = unsafe {
            _mm_cvtps_pd(_mm_rsqrt_ps(_mm_cvtpd_ps(distance_Squared)))  // ~= pow(distance_Squared, -0.5)
        };

        for _ in 0..2 {
            distance_Reciprocal = unsafe {
                _mm_sub_pd(                                                     // = (distance_Reciprocal * 1.5)
                    _mm_mul_pd(distance_Reciprocal, _mm_set1_pd(1.5)),          //     - ((0.5 * distance_Squared * distance_Reciprocal)
                    _mm_mul_pd(                                                 //         * (distance_Reciprocal * distance_Reciprocal))
                        _mm_mul_pd(                                             //
                            _mm_mul_pd(_mm_set1_pd(0.5), distance_Squared),     //
                            distance_Reciprocal),                               //
                        _mm_mul_pd(distance_Reciprocal, distance_Reciprocal)))  //
            };
        }

        // Calculate the magnitudes of force between the bodies.
        magnitudes.as_vectors()[i] = unsafe {
            _mm_mul_pd(                                           // = (0.01 / distance_Squared) * distance_Reciprocal
                _mm_div_pd(_mm_set1_pd(0.01), distance_Squared),  //
                distance_Reciprocal)                              //
        };

    }

    // Use the calculated magnitudes of force to update the bodies' velocities.
    {
        let mut k = 0;
        for i in 0..(BODIES_COUNT - 1) {
            for j in (i + 1)..BODIES_COUNT {
                let i_mass_magnitude = bodies[i].mass * magnitudes.as_scalars()[k];
                let j_mass_magnitude = bodies[j].mass * magnitudes.as_scalars()[k];

                for m in 0..3 {
                    bodies[i].velocity[m] -= position_Deltas[m].as_scalars()[k] * j_mass_magnitude;
                    bodies[j].velocity[m] += position_Deltas[m].as_scalars()[k] * i_mass_magnitude;
                }

                k += 1;
            }
        }
    }

    // Use the updated velocities to update the positions for all of the bodies.
    for i in 0..BODIES_COUNT {
        for m in 0..3 {
            bodies[i].position[m] += 0.01 * bodies[i].velocity[m];
        }
    }
}

/// Calculate the momentum of each body and conserve momentum of the system
/// by adding to the Sun's velocity the appropriate opposite velocity needed
/// in order to offset that body's momentum.
fn offset_Momentum(bodies: &mut [body; BODIES_COUNT]) {
    for i in 0..BODIES_COUNT {
        for m in 0..3 {
            bodies[0].velocity[m] -=
                bodies[i].velocity[m] * bodies[i].mass / SOLAR_MASS;
        }
    }
}

/// Output the total energy of the system.
fn output_Energy(bodies: &mut [body; BODIES_COUNT]) {
    let mut energy = 0.0;
    for i in 0..BODIES_COUNT {
        // Add the kinetic energy for each body.
        energy +=
            0.5
            * bodies[i].mass
            * ( bodies[i].velocity[0] * bodies[i].velocity[0]
              + bodies[i].velocity[1] * bodies[i].velocity[1]
              + bodies[i].velocity[2] * bodies[i].velocity[2]);

        // Add the potential energy between this body and every other body.
        for j in (i + 1)..BODIES_COUNT {
            let mut position_Delta = [0.0; 3];
            for m in 0..3 {
                position_Delta[m] = bodies[i].position[m] - bodies[j].position[m];
            }

            energy -=
                bodies[i].mass * bodies[j].mass
                / f64::sqrt(position_Delta[0] * position_Delta[0]
                          + position_Delta[1] * position_Delta[1]
                          + position_Delta[2] * position_Delta[2]);
        }
    }

    // Output the total energy of the system.
    println!("{:.9}", energy);
}

fn main() {
    let mut solar_Bodies = STARTING_STATE;

    // @Note: It's useful to have two arrays to keep track of the position deltas
    // and magnitudes of force between the bodies for each interaction.
    let mut magnitudes: Interactions =
        Interactions { scalars: [0.0; ROUNDED_INTERACTIONS_COUNT] };

    // @Note: For the `position_Deltas` array, instead of using a 1-D array of structures
    // (each containing X, Y, Z components), a 2-D array is used, consisting of three arrays
    // that each contain all of the X, Y, Z components for all the position deltas.
    let mut position_Deltas: [Interactions; 3] =
        [Interactions { scalars: [0.0; ROUNDED_INTERACTIONS_COUNT] }; 3];

    offset_Momentum(&mut solar_Bodies);
    output_Energy(&mut solar_Bodies);

    let n = std::env::args().nth(1).unwrap().parse().unwrap();
    for _ in 0..n { advance(&mut solar_Bodies, &mut position_Deltas, &mut magnitudes) }
    output_Energy(&mut solar_Bodies);
}
