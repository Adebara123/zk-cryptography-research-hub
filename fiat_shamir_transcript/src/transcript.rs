use sha3::{Keccak256, Digest};
use ark_ff::PrimeField;

// Define data structure
#[derive(Debug, Clone, Default)]
pub struct Transcript {
    hasher: Keccak256,
}

impl Transcript {
    // Constructor for creating a new Transcript instance
    pub fn new() -> Self {
        Self {
            hasher: Keccak256::new(),
        }
    }

    // Method to append new data to the hasher
    pub fn append(&mut self, new_data: &[u8]) {
        self.hasher.update(new_data);
    }

    // Method to sample a challenge from the hasher
    pub fn sample_challenges(&mut self) -> [u8; 32] {
        let mut result = [0_u8; 32];
        let update_data = self.hasher.finalize_reset();
        result.copy_from_slice(&update_data[..32]);
        self.hasher.update(&update_data);
        result
    }

    pub fn sample_challenge<F: PrimeField>(&mut self) -> F {
        let update_data = self.hasher.finalize_reset();
        F::from_random_bytes(&update_data).expect("Failed to convert bytes to field element")
    }

}