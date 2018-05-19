# wbg-rand

Implementation of `rand` for wasm32-unknown-unknown in Rust using
`#[wasm_bindgen]`.

## Usage

First add a dependency to your Cargo.toml:

```toml
# Cargo.toml
[dependencies]
wbg-rand = "0.4"
```

Next add the following to your crate:

```rust
extern crate wbg_rand;

use wbg_rand::{Rng, wasm_rng};
```

The `rand` crate is reexported from the `wbg-rand` crate so the `Rng` trait here
is the [same as it is upstream](https://docs.rs/rand/0.4.2/rand/trait.Rng.html).

And now you use `wasm_rng` just like you would `thread_rng`!

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
