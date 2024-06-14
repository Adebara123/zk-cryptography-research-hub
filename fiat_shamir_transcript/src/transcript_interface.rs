use ark_ff::PrimeField;

pub trait TranscriptInterface {
    fn new() -> Self;
    fn submit_data(&mut self, input_data: &[u8]);
    fn generate_challenge(&mut self) -> [u8; 32];
    fn compute_challenge_in_field<F: PrimeField>(&mut self) -> F;
}