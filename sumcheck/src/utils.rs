use ark_ff::{BigInteger, PrimeField};

// Utility functions
pub fn generate_pairs(len: usize) -> Vec<(usize, usize)> {
    (0..len).step_by(2).map(|i| (i, i + 1)).collect()
}

pub fn boolean_hypercube<F: PrimeField>(n: usize) -> Vec<Vec<F>> {
    let mut result = Vec::new();
    for i in 0..1u128 << n {
        let mut current = Vec::new();
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                current.push(F::one());
            } else {
                current.push(F::zero());
            }
        }
        current.reverse();
        result.push(current);
    }

    result
}


pub fn vec_to_bytes<F: PrimeField>(poly: &Vec<F>) -> Vec<u8> {
    let mut bytes = Vec::new();
    for p in poly {
        bytes.extend_from_slice(&p.into_bigint().to_bytes_be());
    }
    bytes
}
