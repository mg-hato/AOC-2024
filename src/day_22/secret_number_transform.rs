
pub struct SecretNumberTransform {
    modulo: u64
}

impl SecretNumberTransform {
    pub fn new(modulo: u64) -> SecretNumberTransform {
        SecretNumberTransform { modulo }
    }

    pub fn default() -> SecretNumberTransform {
        Self::new(16_777_216)
    }

    /// Evolve secret number `n` times. Returns the sequence of evolution
    pub fn iterative_evolve_sequence(&self, secret_number: u64, n: usize) -> Vec<u64> {
        let mut secret_number = secret_number;
        let mut sequence = Vec::with_capacity(n + 1);
        sequence.push(secret_number);
        for _ in 0..n {
            secret_number = self.evolve(secret_number);
            sequence.push(secret_number);
        }
        sequence
    }

    /// Evolve secret number `n` times
    pub fn iterative_evolve(&self, secret_number: u64, n: usize) -> u64 {
        let mut secret_number = secret_number;
        for _ in 0..n { secret_number = self.evolve(secret_number); }
        secret_number
    }

    pub fn evolve(&self, secret_number: u64) -> u64 {
        // First step
        let secret_number = self.prune(self.mix(secret_number, secret_number * 64));

        // Second step
        let secret_number = self.prune(self.mix(secret_number, secret_number / 32));
        
        // Third step
        let secret_number = self.prune(self.mix(secret_number, secret_number * 2_048));

        secret_number
    }

    pub fn mix(&self, secret_number: u64, value: u64) -> u64 {
        secret_number ^ value
    }

    pub fn prune(&self, secret_number: u64) -> u64 {
        secret_number % self.modulo
    }
}