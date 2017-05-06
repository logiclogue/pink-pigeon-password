use crypto::md5::Md5;
use crypto::digest::Digest;

pub trait Generator {
    fn get(&mut self) -> i32;
    fn get_in_range(&mut self, min: i32, max: i32) -> i32;
}

pub struct SeededGenerator {
    seed: String,
    hash: Md5
}

impl SeededGenerator {
    pub fn new(seed: &str) -> SeededGenerator {
        SeededGenerator {
            seed: seed.to_string(),
            hash: Md5::new()
        }
    }
}

impl Generator for SeededGenerator {
    fn get(&mut self) -> i32 {
        let digest;
        let s: String;
        let num;

        self.hash.input_str(&self.seed);

        digest = self.hash.result_str();

        s = digest
            .chars()
            .take(4)
            .collect();

        num = i32::from_str_radix(&s, 16);

        match num {
            Ok(input_int) => input_int,
            Err(_) => 0
        }
    }

    fn get_in_range(&mut self, min: i32, max: i32) -> i32 {
        let diff = max - min;

        (self.get() % diff) + min
    }
}
