use sgx_rand::Rng;
use sgx_rand::{SeedableRng, XorShiftRng};
use std::time::{Duration, SystemTime};
use std::untrusted::time::SystemTimeEx;

fn from_u128_to_array_of_u32(x: u128) -> [u32; 4] {
    let mut arr = [0; 4];
    for i in 0..4 {
        arr[i] = (x >> (32 * i)) as u32;
    }
    arr
}

pub fn sample(mut val: f32, epsilon: f32, now: SystemTime) -> f32 {
    match now.elapsed() {
        Ok(elapsed) => {
            let seed = elapsed.as_nanos();
            let arr = from_u128_to_array_of_u32(seed);
            let mut rng = XorShiftRng::from_seed(arr);
            let u: f32 = rng.gen();
            let b = 2.0 / epsilon;
            let noise = if u < 0.5 {
                b * u.ln()
            } else {
                -b * (1.0 - u).ln()
            };

            val += noise as f32;
            val
        }
        Err(e) => {
            println!("Error: {e:?}");
            0.0
        }
    }
}
