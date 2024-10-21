use std::ops::{Add, AddAssign};

use ark_ff::PrimeField;
use crate::multilinear_pol::multilinear_poly::MultiLinearPolynomial;


use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct ComposedMultilinearPolynomial<F: PrimeField> {
    pub polys: Vec<MultiLinearPolynomial<F>>,
}

impl<F: PrimeField> ComposedMultilinearPolynomial<F> {
    pub fn new(polys: Vec<MultiLinearPolynomial<F>>) -> Self {
        assert!(!polys.is_empty(), "At least one polynomial is required");
        let variables = polys[0].variables;
        assert!(
            polys.iter().all(|p| p.variables == variables),
            "All polynomials must have the same number of variables"
        );
        Self { polys }
    }

    pub fn evaluate(&self, point: &[F]) -> F {
        assert_eq!(point.len(), self.polys[0].variables, "Incorrect number of evaluation points");
        self.polys.iter().map(|p| p.eval_full(point)).product()
    }

    pub fn partial_eval(&self, eval_point: F, idx: usize) -> Self {
        Self {
            polys: self.polys.iter().map(|p| p.partial_eval(eval_point, idx)).collect(),
        }
    }

    pub fn partial_evaluations(&self, evaluation_points: Vec<F>, variable_indices: Vec<usize>) -> Self {
        Self {
            polys: self.polys.iter().map(|p| p.partial_evaluations(evaluation_points.clone(), variable_indices.clone())).collect(),
        }
    }

    pub fn elementwise_product(&self) -> Vec<F> {
        let eval_len = self.polys[0].evaluations.len();
        (0..eval_len)
            .map(|i| self.polys.iter().map(|p| p.evaluations[i]).product())
            .collect()
    }

    pub fn max_degree(&self) -> usize {
        self.polys.len()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(self.polys.len() as u32).to_le_bytes());
        for poly in &self.polys {
            bytes.extend_from_slice(&poly.to_bytes());
        }
        bytes
    }
}

impl<F: PrimeField> Add for ComposedMultilinearPolynomial<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.polys.len(), other.polys.len(), "Polynomials must have the same length");
        Self {
            polys: self.polys.into_iter().zip(other.polys).map(|(a, b)| a.add(b)).collect(),
        }
    }
}

impl<F: PrimeField> AddAssign for ComposedMultilinearPolynomial<F> {
    fn add_assign(&mut self, other: Self) {
        assert_eq!(self.polys.len(), other.polys.len(), "Polynomials must have the same length");
        for (a, b) in self.polys.iter_mut().zip(other.polys) {
            *a += b;
        }
    }
}