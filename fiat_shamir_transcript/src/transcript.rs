use ark_ff::PrimeField;
use sha3::Keccak256;

use crate::transcript_interface::TranscriptInterface;


pub struct Transcript {
    hasher: Keccak256,
}


impl TranscriptInterface for Transcript {
    fn new() -> Self {
        Self { hasher: Keccak256::new() }
    }

    fn submit_data(&mut self, input_data: &[u8]) {
        self.hasher.update(input_data)
    }

    fn generate_challenge(&mut self) -> [u8; 32] {
        let update_data = self.hasher.finalize_reset();
        self.hasher.update(&update_data);
        let mut result = [0_u8; 32];
        result.copy_from_slice(&update_data);
        result
    }

    fn compute_challenge_in_field<F: PrimeField>(&mut self) -> F {
        F::from_random_bytes(&self.hasher.finalize_reset()).unwrap()
    }
}