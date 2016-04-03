extern crate rand;

use rand::{thread_rng, Rng};

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Robot {
        Robot { name: Robot::generate_name() }
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::generate_name()
    }

    fn generate_name() -> String {
        let letters = (b'A'..b'Z' + 1).collect::<Vec<_>>();
        let numbers = (b'0'..b'9' + 1).collect::<Vec<_>>();
        let mut rng = thread_rng();
        let mut name = String::with_capacity(5);
        for i in 0..5 {
            if i < 2 {
                name.push(*rng.choose(&letters).unwrap() as char);
            } else {
                name.push(*rng.choose(&numbers).unwrap() as char);
            }
        }
        name
    }
}
