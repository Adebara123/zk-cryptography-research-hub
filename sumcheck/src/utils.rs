use ark_ff::Field;

// Utility functions
pub fn boolean_hypercube<F: Field>(num_vars: usize) -> Vec<usize> {
    (0..1 << num_vars).collect()
}

pub fn generate_pairs(len: usize, idx: usize) -> Vec<(usize, usize)> {
    (0..len).step_by(2).map(|i| (i, i + 1)).collect()
}