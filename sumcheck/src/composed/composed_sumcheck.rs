use ark_ff::PrimeField;
use polynomials::composed::composed_multilinear::ComposedMultilinearPolynomial;
use crate::utils::vec_to_bytes;
use fiat_shamir_transcript::transcript::Transcript;

#[derive(Debug, Clone)]
pub struct ComposedSumcheck<F: PrimeField> {
    pub poly: ComposedMultilinearPolynomial<F>,
    pub sum: F,
}

#[derive(Debug, Clone)]
pub struct ComposedSumcheckProof<F: PrimeField> {
    pub round_polys: Vec<Vec<F>>,
    pub final_eval: F,
}

impl<F: PrimeField> ComposedSumcheck<F> {
    pub fn new(poly: ComposedMultilinearPolynomial<F>) -> Self {
        let sum = Self::calculate_sum(&poly);
        ComposedSumcheck { poly, sum }
    }

    pub fn calculate_sum(poly: &ComposedMultilinearPolynomial<F>) -> F {
        poly.elementwise_product().iter().sum()
    }

    pub fn prove(&self) -> (ComposedSumcheckProof<F>, Vec<F>) {
        let mut transcript = Transcript::new();
        let mut current_poly = self.poly.clone();
        let variables = self.poly.polys[0].variables;
        let mut round_polys = Vec::with_capacity(variables);
        let mut challenges = Vec::with_capacity(variables);

        for _ in 0..variables {
            // Calculate round polynomial coefficients
            let round_poly = self.compute_round_poly(&current_poly);
            
            // Commit to round polynomial
            transcript.append(&vec_to_bytes(&round_poly));
            
            // Generate challenge and update state
            let challenge: F = transcript.sample_challenge();
            challenges.push(challenge);
            round_polys.push(round_poly);
            
            // Prepare for next round
            current_poly = current_poly.partial_eval(challenge, 0);
        }

        let final_eval = current_poly.evaluate(&[]);
        
        (
            ComposedSumcheckProof {
                round_polys,
                final_eval,
            },
            challenges,
        )
    }

    fn compute_round_poly(&self, poly: &ComposedMultilinearPolynomial<F>) -> Vec<F> {
        let max_degree = poly.max_degree();
        let mut round_poly = Vec::with_capacity(max_degree + 1);
        
        for i in 0..=max_degree {
            let eval = poly
                .partial_eval(F::from(i as u32), 0)
                .elementwise_product()
                .iter()
                .sum();
            round_poly.push(eval);
        }
        
        round_poly
    }

    pub fn verify(&self, proof: &ComposedSumcheckProof<F>) -> bool {
        let mut transcript = Transcript::new();
        let mut current_sum = self.sum;
        let mut challenges = Vec::new();

        // Verify each round
        for round_poly in &proof.round_polys {
            // Verify claimed sum matches round polynomial evaluation
            if !self.verify_round_consistency(round_poly, current_sum) {
                return false;
            }

            // Generate and store challenge
            transcript.append(&vec_to_bytes(round_poly));
            let challenge: F = transcript.sample_challenge();
            challenges.push(challenge);

            // Update current sum for next round
            current_sum = self.evaluate_uni_poly(round_poly, challenge);
        }

        // Final verification
        self.poly.evaluate(&challenges) == proof.final_eval
    }

    fn verify_round_consistency(&self, round_poly: &[F], claimed_sum: F) -> bool {
        let eval_at_zero = round_poly[0];
        let eval_at_one = self.evaluate_uni_poly(round_poly, F::one());
        eval_at_zero + eval_at_one == claimed_sum
    }

    fn evaluate_uni_poly(&self, coeffs: &[F], point: F) -> F {
        let mut result = F::zero();
        let mut power = F::one();
        
        for &coeff in coeffs {
            result += coeff * power;
            power *= point;
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::Field;
    use ark_test_curves::bls12_381::Fr as F;
    use polynomials::multilinear_pol::multilinear_poly::*;
    
   

  
}
