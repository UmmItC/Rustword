use rand::prelude::*;

pub struct PasswordGenerator {
    charset: Vec<char>,
}

impl PasswordGenerator {
    pub fn new() -> Self {
        let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_=+"
            .chars()
            .collect();
        
        PasswordGenerator { charset }
    }

    pub fn generate(&self, length: usize) -> String {
        let mut rng = thread_rng();
        
        let password: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..self.charset.len());
                self.charset[idx]
            })
            .collect();
        
        password
    }
}