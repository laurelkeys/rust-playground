// http://cliffle.com/p/dangerust/

#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
)]

use std::arch::x86_64::*;
use std::f64::consts::PI;
use std::mem;

#[repr(C)]
struct body {
    position: [f64; 3],
    velocity: [f64; 3],
    mass: f64,
}

const SOLAR_MASS: f64 = 4.0 * PI * PI;
const DAYS_PER_YEAR: f64 = 365.24;
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
unsafe fn output_Energy(bodies: &mut [body; BODIES_COUNT]) {
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
            let mut position_Delta = [mem::MaybeUninit::<f64>::uninit(); 3];
            for m in 0..3 {
                position_Delta[m]
                    .as_mut_ptr()
                    .write(bodies[i].position[m] - bodies[j].position[m]);
            }
            let position_Delta: [f64; 3] = mem::transmute(position_Delta);

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

/// Advance all the bodies in the system by one timestep.
/// Calculate the interactions between all the bodies, update each body's
/// velocity based on those interactions, and update each body's position
/// by the distance it travels in a timestep at it's updated velocity.
unsafe fn advance(bodies: &mut [body; BODIES_COUNT]) {
    const INTERACTIONS_COUNT: usize = BODIES_COUNT * (BODIES_COUNT - 1) / 2;
    const ROUNDED_INTERACTIONS_COUNT: usize = INTERACTIONS_COUNT + INTERACTIONS_COUNT % 2;

    #[repr(align(16))]
    #[derive(Copy, Clone)]
    struct Align16([f64; ROUNDED_INTERACTIONS_COUNT]); // 16 bytes = 128 bits

    static mut position_Deltas: [Align16; 3] = [Align16([0.0; ROUNDED_INTERACTIONS_COUNT]); 3];
    static mut magnitudes: Align16 = Align16([0.0; ROUNDED_INTERACTIONS_COUNT]);

    // Calculate the position_Deltas between the bodies for each interaction.
    {
        let mut k = 0;
        for i in 0..(BODIES_COUNT - 1) {
            for j in (i + 1)..BODIES_COUNT {
                for m in 0..3 {
                    position_Deltas[m].0[k] = bodies[i].position[m] - bodies[j].position[m];
                }

                k += 1;
            }
        }
    }

    // Calculate the magnitudes of force between the bodies for each interaction.
    // @Note: This loop processes two interactions at a time,
    // which is why ROUNDED_INTERACTIONS_COUNT / 2 iterations are done.
    for i in 0..(ROUNDED_INTERACTIONS_COUNT / 2) {
        // Load position_Deltas of two bodies into position_Delta.
        let mut position_Delta = [mem::MaybeUninit::<__m128d>::uninit(); 3];
        for m in 0..3 {
            position_Delta[m]
                .as_mut_ptr()
                .write(
                    *(&position_Deltas[m].0 as *const f64
                                            as *const __m128d)
                        .add(i));
            }
        let position_Delta: [__m128d; 3] = mem::transmute(position_Delta);

        let distance_Squared: __m128d =                                 //
            _mm_add_pd(                                                 // = ((position_Delta[0] * position_Delta[0])
                _mm_add_pd(                                             //     + (position_Delta[1] * position_Delta[1]))
                    _mm_mul_pd(position_Delta[0], position_Delta[0]),   //     + (position_Delta[2] * position_Delta[2])
                    _mm_mul_pd(position_Delta[1], position_Delta[1])),  //
                _mm_mul_pd(position_Delta[2], position_Delta[2]));      //

        // @Todo: Add explanation.
        let mut distance_Reciprocal: __m128d =
        _mm_cvtps_pd(_mm_rsqrt_ps(_mm_cvtpd_ps(distance_Squared)));

        for _ in 0..2 {
            // @Todo: Add explanation.
            distance_Reciprocal =                                                //
                _mm_sub_pd(                                                      // = (distance_Reciprocal * 1.5)
                    _mm_mul_pd(distance_Reciprocal, _mm_set1_pd(1.5)),           //     - ((0.5 * distance_Squared * distance_Reciprocal)
                    _mm_mul_pd(                                                  //         * (distance_Reciprocal * distance_Reciprocal))
                        _mm_mul_pd(                                              //
                            _mm_mul_pd(_mm_set1_pd(0.5), distance_Squared),      //
                            distance_Reciprocal),                                //
                        _mm_mul_pd(distance_Reciprocal, distance_Reciprocal)));  //
        }

        // @Todo: Add explanation.
        (magnitudes.0.as_mut_ptr() as *mut __m128d)
            .add(i)
            .write(
                _mm_mul_pd(                                           //
                    _mm_div_pd(_mm_set1_pd(0.01), distance_Squared),  // = (0.01 / distance_Squared) * distance_Reciprocal
                    distance_Reciprocal));                            //

    }

    // Use the calculated magnitudes of force to update the bodies' velocities.
    {
        let mut k = 0;
        for i in 0..(BODIES_COUNT - 1) {
            for j in (i + 1)..BODIES_COUNT {
                let i_mass_magnitude = bodies[i].mass * magnitudes.0[k];
                let j_mass_magnitude = bodies[j].mass * magnitudes.0[k];

                for m in 0..3 {
                    bodies[i].velocity[m] -= position_Deltas[m].0[k] * j_mass_magnitude;
                    bodies[j].velocity[m] += position_Deltas[m].0[k] * i_mass_magnitude;
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

fn main() {
    unsafe {
        offset_Momentum(&mut solar_Bodies);
        output_Energy(&mut solar_Bodies);

        let n = std::env::args()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        for _ in 0..n { advance(&mut solar_Bodies) }
        output_Energy(&mut solar_Bodies);
    }
}
