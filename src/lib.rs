//! A crate to enable usage of the `rand` crate on wasm32-unknown-unknown
//!
//! This crate provides two types, a `MathRandomRng` and a `WasmRng`. The
//! `MathRandomRng` is a random number generated which is purely backed by
//! `Math.random`. The `WasmRng` is a faster in-was RNG which draws entropy
//! periodically from `Math.random`.
//!
//! Users of `rand::thread_rng` should feel right at home with `wasm_rng`.
//!
//! # Example Usages
//! See the Rng trait for more examples: https://docs.rs/rand/0.5.0-pre.2/rand/trait.Rng.html.
//!
//! ```no_run
//! use wbg_rand::{Rng, wasm_rng, math_random_rng};
//!
//! // get random boolean, `math_random_rng()` samples `Math.random` in JS every call
//! let a: bool = math_random_rng().gen();
//! println!("{}", a);
//!
//! // `wasm_rng()` only samples `Math.random` to re-seed periodically
//! let n = wasm_rng().gen::<f64>();
//! println!("{}", n);
//!
//! let r: usize = wasm_rng().gen_range(0, 10);
//! println!("{}", r);
//! ```

#![feature(proc_macro, wasm_custom_section, wasm_import_module, use_extern_macros)]
extern crate rand;
extern crate wasm_bindgen;
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use wasm_bindgen::prelude::*;

pub use rand::*;

/// A random number generated which is hooked up to `Math.random` in JS.
pub struct MathRandomRng(());

pub fn math_random_rng() -> MathRandomRng {
    MathRandomRng(())
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

impl Rng for MathRandomRng {
    fn next_u32(&mut self) -> u32 {
        (random() * (u32::max_value() as f64)) as u32
    }
}

/// A reasonably fast RNG for wasm which draws entropy from `Math.random`.
pub struct WasmRng(());

pub fn wasm_rng() -> WasmRng {
    WasmRng(())
}

lazy_static! {
    static ref RNG: Mutex<reseeding::ReseedingRng<IsaacRng, WasmRngReseed>> = {
        Mutex::new(reseeding::ReseedingRng::new(math_random_rng().gen(), 32, WasmRngReseed))
    };
}

struct WasmRngReseed;

impl reseeding::Reseeder<IsaacRng> for WasmRngReseed {
    fn reseed(&mut self, rng: &mut IsaacRng) {
        *rng = math_random_rng().gen();
    }
}

impl Rng for WasmRng {
    fn next_u32(&mut self) -> u32 {
        RNG.lock()
            .unwrap_or_else(|e| e.into_inner())
            .next_u32()
    }
    fn next_u64(&mut self) -> u64 {
        RNG.lock()
            .unwrap_or_else(|e| e.into_inner())
            .next_u64()
    }
    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        RNG.lock()
            .unwrap_or_else(|e| e.into_inner())
            .fill_bytes(bytes)
    }
}
