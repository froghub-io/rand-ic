pub struct Rand {
    pub seed: u128,
    pub a: u128,
    pub c: u128,
    pub m: u128,
}

impl Rand {
    pub fn new() -> Rand{
        let now = if cfg!(all(target_arch = "wasm32", target_os = "unknown")) {
            (ic_cdk::api::time() / 1000000000 ) as u128
        } else {
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u128
        };
        Rand{seed: now, a: 0xBC8F, c: 0xB, m: (1 << 31) - 1}
    }

    pub fn rand(&mut self) {
        self.seed = (self.seed * self.a + self.c) & self.m;
    }

    pub fn next(&mut self, bound: usize) -> usize {
        self.rand();
        (self.seed % (bound as u128)) as usize
    }

    pub fn fill_i8(&mut self, dest: &mut [i8]) {
        let data = (0..dest.len()).map(|_| {
            self.rand();
            (self.seed % 256) as i8
        }).collect::<Vec<_>>();
        dest.copy_from_slice(&data);
    }

    pub fn fill_u8(&mut self, dest: &mut [u8]) {
        let data = (0..dest.len()).map(|_| {
            self.rand();
            (self.seed % 256) as u8
        }).collect::<Vec<_>>();
        dest.copy_from_slice(&data);
    }
}

#[test]
fn test() {
    let mut rng = Rand::new();
    let mut buf = [0u8; 32];
    rng.fill_u8(&mut buf);
    println!("{:?}", buf);
}