use polynomials::multilinear_pol::multilinear_poly::MultiLinearPolynomial;
use ark_ff::{Field, PrimeField, BigInteger};
use fiat_shamir_transcript::transcript::Transcript;
use crate::utils::*;

// SumCheckProof structure
#[derive(Debug, Clone, PartialEq)]
pub struct SumCheckProof<F: PrimeField> {
    polynomial: MultiLinearPolynomial<F>,
    sum: F,
    round_polynomials: Vec<MultiLinearPolynomial<F>>,
    initial_round_polynomial: MultiLinearPolynomial<F>,
}

impl<F: PrimeField> SumCheckProof<F> {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = vec![];
        result.extend(self.polynomial.to_bytes());
        result.extend(self.sum.into_bigint().to_bytes_be());
        for poly in &self.round_polynomials {
            result.extend(poly.to_bytes());
        }
        result.extend(self.initial_round_polynomial.to_bytes());
        result
    }
}


// Prover structure and implementation
#[derive(Clone, Default, Debug)]
pub struct Prover<F: PrimeField> {
    poly: MultiLinearPolynomial<F>,
    round_polynomials: Vec<MultiLinearPolynomial<F>>,
    initial_round_polynomial: MultiLinearPolynomial<F>,
    sum: F,
    transcript: Transcript,
}

impl<F: PrimeField> Prover<F> {
    pub fn new(poly: MultiLinearPolynomial<F>) -> Self {
        Self {
            poly,
            round_polynomials: Default::default(),
            initial_round_polynomial: Default::default(),
            sum: Default::default(),
            transcript: Transcript::new(),
        }
    }

    pub fn new_with_sum(poly: MultiLinearPolynomial<F>, sum: F) -> Self {
        Self {
            poly,
            round_polynomials: Default::default(),
            initial_round_polynomial: Default::default(),
            sum,
            transcript: Transcript::new(),
        }
    }

    pub fn calculate_sum(&mut self) {
        self.sum = self.poly.evaluations.iter().sum();
    }

    pub fn compute_initial_round_polynomial(&mut self) {
        let number_of_round = self.poly.variables - 1; // We need the full set of variables for the boolean hypercube
        let bh = boolean_hypercube::<F>(number_of_round);
        let mut bh_partials: MultiLinearPolynomial<F> = MultiLinearPolynomial::zero(1);
        
        for bh_i in bh {
            let current_partial = self.poly.partial_evaluations(bh_i, vec![1; number_of_round]); // Always evaluate at 0th index for the sumcheck
                bh_partials += current_partial;
            
        }

        self.transcript.append(&bh_partials.to_bytes());
        self.initial_round_polynomial = bh_partials;
    }



    pub fn generate_sumcheck_proof(&mut self) -> SumCheckProof<F> {
        self.compute_initial_round_polynomial();
        let mut all_random_responses = Vec::new();

        let mut current_poly = self.poly.clone();

        for _ in 0..self.poly.variables {
            let verifier_random_response = self.transcript.sample_challenge();
            all_random_responses.push(verifier_random_response);

            let next_poly = current_poly.partial_eval(verifier_random_response, 0);
            self.transcript.append(&next_poly.to_bytes());
            self.round_polynomials.push(next_poly.clone());

            current_poly = next_poly;
        }

        SumCheckProof {
            polynomial: self.poly.clone(),
            round_polynomials: self.round_polynomials.clone(),
            initial_round_polynomial: self.initial_round_polynomial.clone(),
            sum: self.sum,
        }
    }


}


#[cfg(test)]
mod tests {

    use super::*;

    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_sum_calculation() {
        let poly = MultiLinearPolynomial::new(
            3,
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(4),
            ],
        );
        let mut prover = Prover::new(poly);
        prover.calculate_sum();
        assert_eq!(prover.sum, Fr::from(12));
    }


    #[test]
    fn test_compute_intial_round_poly() {
        let poly = MultiLinearPolynomial::new(
            3,
            vec![
                Fr::from(0),
                Fr::from(0),
                Fr::from(0),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(2),
                Fr::from(4),
            ],
        );
        let mut prover = Prover::new(poly);
        prover.compute_initial_round_polynomial();
        assert_eq!(
            prover.initial_round_polynomial.evaluations,
            vec![Fr::from(2), Fr::from(10)]
        );
    }


}