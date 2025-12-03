#[derive(Debug)]
struct GMRand {
    seed: u32,
    state: [u32; 16],
    index: usize,
    random_poly: u32,
    // state&mask,
}

impl GMRand {
    // move seeding to separate function?
    // add option for "correct" seeding
    fn new(seed: u32) -> Self {
        let mut state = [0;16];
        let mut temp = seed;

        for i in 0..16 {
            temp = (temp * 0x343fd + 0x269ec3) >> 16;
            state[i] = temp;
        }

        GMRand {
            seed,
            state,
            index: 0,
            random_poly: 0xDA442D24,
        }
    }

    fn init_random(mut self: Self, seed: u32) {
        
    }

    // YYRandom in GML source code
    fn _get_rand(mut self: Self) -> u32 {
        let t1 = self.state[self.index] ^ self.state[(self.index + 0xd) & 0xf];
        let t2 = t1 ^ (self.state[self.index] << 0x10) ^ (self.state[(self.index + 0xd) & 0xf] << 0xf);
        let t3 = self.state[(self.index + 9) & 0xf] ^ (self.state[(self.index + 9) & 0xf] >> 0xb);
        let t4 = t2 ^ t3;
        self.state[self.index] = t4;
        self.index = (self.index - 1) & 0xf;
        self.state[self.index] = 1;//wtf
        self.state[self.index]
    }
}

fn main() {
    let r = GMRand::new(123);
    dbg!(r);
    println!("Hello, world!");
}
