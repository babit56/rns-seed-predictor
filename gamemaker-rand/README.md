# Gamemaker RNG

Gamemaker RNG functions, implemented in Rust. Functions accept and return `Real`s, which are (un)signed integers or floats.

Currently the main implementation is a WELL512a impl based on reverse engineering of an empty YYC binary on Windows and [rerand](https://gist.github.com/nkrapivin/ea4db4abb8a1994c0c1ba88f54196fa6). The original implementation by Gamemaker is based on [Lomont 2008](https://lomont.org/papers/2008/Lomont_PRNG_2008.pdf), see: https://github.com/YoYoGames/GameMaker-Bugs/issues/5695#issuecomment-2091076398

I have not thoroughly checked that the same impl is used for all or any other platforms, but preliminary testing indicates at least the Windows VM and Linux VM runtimes use it. I also think HTML uses it, but the implementation details are subtly different, so I can't say for sure without testing. JS shenanigans also don't help!

The WELL512a is implemented wrong (specifically `set_seed()`), so for educational purposes a "correct" impl is also included (see `WELL512a::with_correct_tlcg()`).

Other than that I am not familiar with any standard RNG's in the Gamemaker community, but I'm open to adding them if wanted.

Issues or pull requests are welcome!

## Usage

```rust
// Bring RNG functions into scope
use gamemaker_rand::GMRand;

// Instantiate the default RNG (currently Windows based)
let rand = gamemaker_rand::rng();

// Functions accept and return Real's, which requires quite a few .into()'s
rand.set_seed(1729.into());
let dice_roll: usize = rand.irandom(6.into()).into();
```

## Building

```sh
cargo build -p gamemaker-rand
```

## TODO
- [x] WELL512a implementation
- [ ] Tests against Windows/Linux
- [ ] Tests against HTML, maybe others
- [ ] Finish docs
- [ ] More examples?
