use rand::{prelude::ThreadRng, distributions::Uniform, Rng};

#[derive(Clone)]
pub struct RandomByte {
    rng: ThreadRng,
    side: Uniform<u8>,
}

impl RandomByte {
    pub fn new() -> RandomByte {
        RandomByte {
            rng: rand::thread_rng(),
            side: Uniform::new(0, 255),
        }
    }

    pub fn sample(&mut self) -> u8 {
        self.rng.sample(self.side)
    }
}