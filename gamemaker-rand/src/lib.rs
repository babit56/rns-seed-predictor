//! Simulate Gamemaker RNG

// TODO: better docs, especially the structure of them
// TODO: check how GM does casting of RValues
// TODO: notes on well512a using u32 values internally
// TODO: Make example "correct" version, html version, maybe more

/// Corresponds to the so-called "Real Numbers" from Gamemaker.
/// This is not an accurate representation of how Real works in GM, but it suffices for letting e.g. [`GMRand::random()`] take multiple different types
#[derive(Debug, Clone, Copy)]
pub enum Real {
    Float(f64),
    Unsigned(u64),
    Signed(i64),
}

// Implementing casting to/from various integer types
macro_rules! impl_casting {
    ($t0:ty, $variant:expr, $t1:ty) => {
        impl From<$t0> for Real {
            fn from(value: $t0) -> Self {
                $variant(value as $t1)
            }
        }
        impl Into<$t0> for Real {
            fn into(self) -> $t0 {
                match self {
                    Real::Float(n) => n as $t0,
                    Real::Unsigned(n) => n as $t0,
                    Real::Signed(n) => n as $t0,
                }
            }
        }
    };
}

impl_casting!(f32, Real::Float, f64);
impl_casting!(f64, Real::Float, f64);
impl_casting!(u32, Real::Unsigned, u64);
impl_casting!(u64, Real::Unsigned, u64);
impl_casting!(usize, Real::Unsigned, u64);
impl_casting!(i32, Real::Signed, i64);
impl_casting!(i64, Real::Signed, i64);
impl_casting!(isize, Real::Signed, i64);

/// Exposes the same functions related to randomness that Gamemaker does. Uses `Real` as the type for interacting with all member functions
///
/// Currently only implemented by `WELL512a`
pub trait GMRand {
    fn get_seed(&self) -> Real;
    fn set_seed(&mut self, seed: Real);
    fn random(&mut self, upper: Real) -> Real;
    fn random_range(&mut self, lower: Real, upper: Real) -> Real;
    fn irandom(&mut self, upper: Real) -> Real;
    fn irandom_range(&mut self, lower: Real, upper: Real) -> Real;
    fn choose<'list, T>(&mut self, list: &'list [T]) -> Option<&'list T>;
    fn ds_list_shuffle<T>(&mut self, _list: &mut [T]) {
        unimplemented!()
    }
    fn ds_grid_shuffle<T>(&mut self, _grid: &mut [T]) {
        unimplemented!()
    }
    fn array_shuffle<T: Clone>(&mut self, array: &[T]) -> Vec<T> {
        let mut out = array.to_vec();
        self.ds_list_shuffle(&mut out);
        out
    }
    fn array_shuffle_ext<T>(&mut self, array: &mut [T]) {
        self.ds_list_shuffle(array);
    }
    /// Set seed to a "random" value (usually derived from clock or similar). Deemed irrelevant to implement, but included for completeness
    fn randomise(&mut self) {
        unimplemented!()
    }
}

/// The current Gamemaker RNG.
///
/// The main PRNG is the WELL512a algorithm, in particular the implementation from Lomont 2008[^lomont].
/// Seeding via [`set_seed()`](WELL512a::set_seed) is done by a TLCG, also from Lomont 2008. Seeding via `randomize()` takes the current time, and feeds it into the same TLCG..
///
/// The TLCG is however implemented wrong, leading to there only being 2^16 unique seeds rather than 2^32. See [`_init_random()`](WELL512a::_init_random) for more details.
///
/// See also [`new_legacy()`](WELL512a::new_legacy) for information on some differences between modern and legacy versons of GML.
///
/// [^lomont]: <https://lomont.org/papers/2008/Lomont_PRNG_2008.pdf>
#[derive(Debug, Clone, Copy)]
pub struct WELL512a {
    pub seed: u32,
    pub state: [u32; 16],
    pub index: usize,
    random_poly: u32,
    tlcg_mask: u32,
    correct_tlcg: bool,
}

impl WELL512a {
    /// Instanstiate a [`WELL512a`] struct
    pub fn new() -> Self {
        Self {
            seed: 0,
            state: [0; 16],
            index: 0,
            random_poly: 0xDA442D24,
            tlcg_mask: u32::MAX,
            correct_tlcg: false,
        }
    }

    /// Instanstiate a [`WELL512a`] struct with legacy variables
    ///
    /// In very old versions of Gamemaker, there were two variables relating to randomization that were allegedly different.
    /// 1. There was a typo or bad implementation in the Lomont 2008 paper were the literal `0xDA442D20` was used instead of `0xDA442D24`
    /// 2. At the end of the TLCG used in seeding, they mask with `0x7fffffff`, which is almost what is done in the TLCG in Lomont 2008.
    /// Notice however that thanks to the bitshift happening right before the mask, this is actually a noop. In modern GML, it has either been removed or is being optimized out by the compiler.
    ///
    /// To be perfectly clear: the mask should be `0x7fff` if they are going for the TLCG from Lomont 2008.
    ///
    /// You can see remains of both of these in the open source HTML runtime[^html_source]. It seems the relevant code was created (and likely last updated) mid-2011.
    /// See `random_use_old_version()` that changes the first literal back to it's old value for compatability.
    /// See also `InitRandom()` which still has the bitmask. It's possible it's there for a JS trick.
    /// You can see the correct `0x7fff` mask in the variable RAND_MAX, but this variable is not used anywhere. mask in the variable RAND_MAX, but this variable is not used anywhere.
    ///
    /// [^html_source]: <https://github.com/YoYoGames/GameMaker-HTML5/blob/develop/scripts/functions/Function_Maths.js>
    pub fn with_legacy() -> Self {
        Self {
            random_poly: 0xDA442D20,
            tlcg_mask: 0x7fffffff,
            ..Self::new()
        }
    }

    pub fn with_correct_tlcg() -> Self {
        Self {
            correct_tlcg: true,
            ..Self::new()
        }
    }

    /// Initialize WELL512a based on a 32-bit unsigned integer.
    ///
    /// Seemingly uses the TLCG shown in Lomont 2008 [^1], which is a TLCG which is generally used a lot in Microsoft products.
    /// However, a correct implementation would do the following:
    /// 1. Seed the TLCG with the seed provided.
    /// 2. Iterate the state with the LCG
    /// 3. For each iteration, output the upper bits of the LCG output (bits 16-30 in Lomont)
    ///
    /// The Gamemaker implementation does the truncation *when iterating the state*,
    /// which means only the upper bits are used for seeding WELL512a. Additionally, they seemingly use the wrong bitmask (`0x7fffffff` instead of `0x7fff`),
    /// so they actually use bits 16-32. I don't know how much the latter bug/choice matters, but the first bug/choice means that there's only 2^16 possible initial states for WELL512a, i.e. 2^16 unique seeds.
    /// This was reported on their bugtracker back in 2003[^2].
    ///
    /// [^1]: <https://lomont.org/papers/2008/Lomont_PRNG_2008.pdf>
    /// [^2]: <https://github.com/YoYoGames/GameMaker-Bugs/issues/3006>
    pub fn _init_random(self: &mut Self, seed: u32) -> u32 {
        self.seed = seed;
        self.index = 0;

        let mut temp = seed;
        for i in 0..16 {
            // temp = (temp.wrapping_mul(0x343fd).wrapping_add(0x269ec3)) >> 16 & self.tlcg_mask;
            temp = (temp.wrapping_mul(214013).wrapping_add(2531011)) >> 16 & self.tlcg_mask;
            self.state[i] = temp;
        }
        seed
    }

    /// InitRandom but with correct usage of the TLCG
    /// The TLCG is literally just the one from Lomont 2008, i.e. the MS algo for rand()
    fn _init_random_correct(self: &mut Self, seed: u32) -> u32 {
        self.seed = seed;
        self.index = 0;

        let mut temp = seed;
        for i in 0..16 {
            temp = temp.wrapping_mul(214013).wrapping_add(2531011);
            self.state[i] = (temp >> 16) & 0x7FFF;
        }
        seed
    }

    // YYRandom in GML source code
    fn _get_rand(self: &mut Self) -> u32 {
        // Taken from https://lomont.org/papers/2008/Lomont_PRNG_2008.pdf
        let (mut a, b, mut c, d);
        a = self.state[self.index];
        c = self.state[(self.index + 13) & 15];
        b = a ^ c ^ (a << 16) ^ (c << 15);
        c = self.state[(self.index + 9) & 15];
        c ^= c >> 11;
        self.state[self.index] = b ^ c;
        a = self.state[self.index];
        d = a ^ ((a << 5) & self.random_poly);
        self.index = (self.index + 15) & 15;
        a = self.state[self.index];
        self.state[self.index] = a ^ b ^ d ^ (a << 2) ^ (b << 18) ^ (c << 28);
        return self.state[self.index];

        // Adapted from rerand: https://gist.github.com/nkrapivin/ea4db4abb8a1994c0c1ba88f54196fa6
        // let t1 = self.state[self.index] ^ self.state[(self.index + 0xd) & 0xf];
        // let t2 =
        //     t1 ^ (self.state[self.index] << 0x10) ^ (self.state[(self.index + 0xd) & 0xf] << 0xf);
        // let t3 = self.state[(self.index + 9) & 0xf] ^ (self.state[(self.index + 9) & 0xf] >> 0xb);
        // let t4 = t2 ^ t3;
        // self.state[self.index] = t4;
        // self.index = (self.index - 1) & 0xf;
        // self.state[self.index] = self.state[self.index]
        //     ^ t2
        //     ^ (t4 ^ (self.random_poly & (t4 << 5)))
        //     ^ (self.state[self.index] << 2)
        //     ^ (t1 << 0x12)
        //     ^ (t3 << 0x1c);
        // self.state[self.index]
    }

    fn _get_rand_upper(self: &mut Self, upper: u32) -> u32 {
        self._get_rand() % upper
    }

    // iScript_Random in GML source code
    fn _get_rand_long(self: &mut Self, upper: i64) -> i64 {
        let rand1 = self._get_rand() as i64;
        let rand2 = self._get_rand() as i64;
        let wide_rand = rand1 | (rand2 << 32) & 0x7fffffffffffffff;
        if upper == 0 {
            wide_rand
        } else {
            upper.signum() * wide_rand % upper.abs()
        }
    }

    // fn iscript_random(self: &mut Self, upper: i64) -> i64 {
    //     let rand1 = self._get_rand() as i64;
    //     let rand2 = self._get_rand() as i64;
    //     let long_rand = (rand1 & 0xffffffff) | ((rand2 & 0x7fffffff) << 32);
    //     let mut signum = -1;
    //     if -1 < upper {
    //         // if upper is 0 or positive
    //         signum = 1
    //     }
    //     let nonneg_upper = signum * upper;
    //     if nonneg_upper == 0 {
    //         long_rand * signum
    //     } else {
    //         (long_rand - long_rand / nonneg_upper * nonneg_upper) * signum
    //     }
    // }

    // fn yyrandomrange(self: &mut Self, mut upper: f64) -> f64 {
    //     upper = upper.floor();
    //     let rand = self._get_rand() as f64;
    //     let v3 = upper.abs();
    //     let mut v1 = 0.0;
    //     if v3 != 0.0 {
    //         v1 = rand.div_euclid(v3);
    //     }
    //     rand - v1 * v3
    // }
}

impl GMRand for WELL512a {
    fn get_seed(&self) -> Real {
        (self.seed as u64).into()
    }

    fn set_seed(self: &mut Self, seed: Real) {
        let seed_inner: i64 = seed.try_into().unwrap();
        if self.correct_tlcg {
            self._init_random_correct(seed_inner as u32);
        } else {
            self._init_random(seed_inner as u32);
        }
    }

    // TODO: test what happens when NaNs are given
    fn random(self: &mut Self, upper: Real) -> Real {
        let upper: f64 = upper.into();
        let rand = self._get_rand() as f64;
        Real::Float((rand / 4294967296.0) * upper)
    }

    fn random_range(self: &mut Self, lower: Real, upper: Real) -> Real {
        let lower: f64 = lower.into();
        let upper: f64 = upper.into();
        let diff = (upper - lower).abs();
        let rand: f64 = self.random(Real::Float(1.0)).into();
        Real::Float(lower.min(upper) + rand * diff)
    }

    fn irandom(self: &mut Self, upper: Real) -> Real {
        let upper: i64 = upper.try_into().unwrap();
        let signum = if upper < 0 { -1 } else { 1 };
        // self._get_rand_long(upper + signum) as u32 as f64
        self._get_rand_long(upper + signum).into()
    }

    fn irandom_range(&mut self, lower: Real, upper: Real) -> Real {
        let lower: i64 = lower.try_into().unwrap();
        let upper: i64 = upper.try_into().unwrap();
        let diff = (upper - lower).abs();
        let rand = self._get_rand_long(diff + 1);
        // (lower.min(upper) + rand) as u32 as f64
        (lower.min(upper) + rand).into()
    }

    fn choose<'l, T>(self: &mut Self, list: &'l [T]) -> Option<&'l T> {
        if list.len() == 0 {
            return None;
        }
        let rand = self._get_rand_upper(list.len() as u32);
        // Top 10 most cursed type castings
        let floored = (rand as f32).floor() as u32 as usize;
        if floored < list.len() {
            Some(&list[floored])
        } else {
            Some(&list[list.len() - 1])
        }
    }

    fn ds_list_shuffle<T>(self: &mut Self, list: &mut [T]) {
        let len = list.len() as u32;
        for _ in 0..8 * list.len() {
            let index1 = self._get_rand_upper(len) as usize;
            let index2 = self._get_rand_upper(len) as usize;
            list.swap(index1, index2);
        }
    }
}

/// Gets the currently used Gamemaker RNG on Windows machines
pub fn rng() -> WELL512a {
    WELL512a::new()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
