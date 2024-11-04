use polynomials::multilinear_pol::multilinear_poly::MultiLinearPolynomial;
use crate::prover::*;
use ark_ff::PrimeField;
use fiat_shamir_transcript::transcript::Transcript;

#[derive(Clone, Default, Debug)]
pub struct Verifier<F: PrimeField> {
    transcript: Transcript,
    phantom: std::marker::PhantomData<F>,
}

impl<F: PrimeField> Verifier<F> {
    pub fn new() -> Self {
        Self {
            transcript: Transcript::new(),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn verify(&mut self, proof: &SumCheckProof<F>) -> bool {
        let mut random_challenges = Vec::new();

        let computed_sum = proof.initial_round_polynomial.eval_full(&vec![F::one()])
            + proof.initial_round_polynomial.eval_full(&vec![F::zero()]);

        if computed_sum != proof.sum {
            println!("Computed sum does not match proof sum: {} != {}", computed_sum, proof.sum);
            return false;
        }

        self.transcript.append(&proof.initial_round_polynomial.to_bytes());

        for i in 0..proof.round_polynomials.len() {
            let random_challenge = self.transcript.sample_challenge();
            random_challenges.push(random_challenge);

            let current_round_poly = &proof.round_polynomials[i];
            let prev_round_poly_eval = if i == 0 {
                proof.initial_round_polynomial.eval_full(&vec![random_challenge])
            } else {
                proof.round_polynomials[i - 1].eval_full(&vec![random_challenge])
            };

            let current_round_poly_eval_sum = current_round_poly.eval_full(&vec![F::one()])
                + current_round_poly.eval_full(&vec![F::zero()]);

            if prev_round_poly_eval != current_round_poly_eval_sum {
                println!(
                    "Polynomial evaluation mismatch at round {}: {} != {}",
                    i, prev_round_poly_eval, current_round_poly_eval_sum
                );
                return false;
            }

            self.transcript.append(&current_round_poly.to_bytes());
        }

        let final_challenge = self.transcript.sample_challenge();
        random_challenges.push(final_challenge);

        let last_poly_eval = proof.round_polynomials.last().unwrap()
            .eval_full(&vec![final_challenge]);
        let main_poly_eval = proof.polynomial.eval_full(&random_challenges);

        if last_poly_eval != main_poly_eval {
            println!(
                "Final polynomial evaluation mismatch: {} != {}",
                last_poly_eval, main_poly_eval
            );
            return false;
        }

        true
    }
}
