use rand::{distributions::Uniform, prelude::ThreadRng, Rng};

#[derive(Clone, Debug)]
pub struct RandomByte {
    rng: ThreadRng,
    distr: Uniform<u16>,
}

impl RandomByte {
    pub fn new() -> RandomByte {
        RandomByte {
            rng: rand::thread_rng(),
            distr: Uniform::new(0, 256),
        }
    }

    pub fn sample(&mut self) -> u8 {
        self.rng.sample(self.distr) as u8
    }
}
