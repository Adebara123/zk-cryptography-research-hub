
use ark_ff::Field;
use ark_ff::PrimeField;

use super::utiles::generate_pairs;


// The multilinear implementation follows ther assumed eveluation by the boolean hypercube 

#[derive(Clone, Debug, PartialEq)]

pub struct MultiLinearPolynomial<F: Field> {
    //veriables is the amount of variables in the poly
    variables: usize,

    // array of evaluations from the boolean hypercube for the said poly
    evaluations: Vec<F>
}

impl <F: PrimeField> MultiLinearPolynomial<F> {
    pub fn new( _variables: usize, evaluations: Vec<F>) -> Self{
        
        // Also checks that the length is not zero 
        assert!(evaluations.len() > 0 as usize, "evaluations cannot be zero");

        // this check that the length of the evaluations is equal to 2 to the power of number of variables
        assert!(evaluations.len() == (2usize.pow(_variables as u32)), "wrong length of eval");


        Self {
            variables: _variables,
            evaluations: evaluations
        }
    }


    pub fn add(&self, other: Self) -> Self {

        let eval_1_len = self.evaluations.len();
        let eval_2_len = other.evaluations.len();

        let mut new_eval: Vec<F> = Vec::with_capacity(eval_1_len);

        if self.variables == other.variables {
            
            for i in 0..eval_1_len {
                let add = self.evaluations[i] + other.evaluations[i];
                new_eval.push(add);
            }

            return Self {
                variables: self.variables,
                evaluations: new_eval
            };

        }

        // get the length of them both and determine the longer one

        let (longer_eval, shorter_eval, longer_len, shorter_len, higher_variable) = if eval_1_len > eval_2_len {
            (&self.evaluations, &other.evaluations, eval_1_len, eval_2_len, self.variables)
        } else {
            ( &other.evaluations, &self.evaluations, eval_2_len, eval_1_len, other.variables)
        };

        let mut new_longer_eval = vec![F::zero(); longer_len as usize];

        for (i, &val) in shorter_eval.iter().enumerate() {

            let factor = longer_len / shorter_len;

            for j in 0..factor {
                new_longer_eval[i * factor + j] = val;
            }
        }

        // Create a new array for the new sum 

        let mut sum_result = vec![F::zero(); longer_len as usize];

        // sum them up 

        for i in 0..longer_len {
            sum_result[i] =longer_eval[i] + new_longer_eval[i];
         
        }        


        Self::new(higher_variable, sum_result)

    }


    pub fn partial_eval(&self, eval_point: F, idx: usize) -> Self {
        let evals_ref = &self.evaluations;
        let mut new_results = Vec::with_capacity(evals_ref.len() / 2);

        for (first, second) in generate_pairs(evals_ref.len(), idx) {
            let val1 = &evals_ref[first];
            let val2 = &evals_ref[second];

            let combined_result: F = (eval_point * val2) + (F::one() - eval_point) * val1;
            new_results.push(combined_result);
        }

        Self { 
            variables: self.variables - 1, 
            evaluations: new_results 
        }
    }

    

    pub fn eval_full(&self, eval_points: &[F]) -> F {
        assert_eq!(eval_points.len(), self.variables, "Length of eval_points must equal number_of_variables");

        let mut final_result: F = F::one();
        let mut current_eval = self.clone();

        for &point in eval_points.iter() {
            current_eval = current_eval.partial_eval(point, 0);
        }
        
        final_result = current_eval.evaluations[0];
        final_result
    }

}


#[cfg(test)]
mod tests {


    use std::vec;

    use crate::multilinear_pol::multilinear_poly::*;

    use ark_test_curves::bls12_381::Fr;

    type F = Fr;
    type poly = MultiLinearPolynomial<F>;


    #[test] 

    fn test_equal_length_evaluatios () {

        let eval_1 = poly::new(2, vec![F::from(1), F::from(2), F::from(3), F::from(4)]);

        let eval_2 = poly::new(2,vec![F::from(1), F::from(2), F::from(3), F::from(4)]);

        assert_eq!(eval_1.add(eval_2), poly::new(2, vec![F::from(2), F::from(4), F::from(6), F::from(8)]));
    }


    #[test] 
    fn test_different_length_evaluation () {
        let eval_1 = poly::new(3, vec![F::from(1), F::from(2), F::from(3), F::from(4), F::from(1), F::from(2), F::from(3), F::from(4)]);

        let eval_2 = poly::new(2,vec![F::from(1), F::from(2), F::from(3), F::from(4)]);

        assert_eq!(eval_1.add(eval_2), poly::new(3, vec![F::from(2), F::from(3), F::from(5), F::from(6), F::from(4), F::from(5), F::from(7), F::from(8)]));
    }

    #[test]
    fn test_partial_evaluation(){
        let evaluations = vec![F::from(0),F::from(0),F::from(2),F::from(5)];
        let polynomial = MultiLinearPolynomial::new(2,evaluations);
        let evaluation_point = F::from(3);
        let new_polynomial = MultiLinearPolynomial::partial_eval(&polynomial, evaluation_point,0);
        assert_eq!(new_polynomial.evaluations, vec![F::from(6), F::from(15)]);
    }


    #[test]
    fn test_full_evalaution() {
        let evaluations = vec![F::from(0),F::from(0),F::from(2),F::from(5)];
        let polynomial = MultiLinearPolynomial::new(2,evaluations);
        let eval_points = vec![F::from(3), F::from(5)];
        let result = MultiLinearPolynomial::eval_full(&polynomial, &eval_points);
        assert_eq!(result, F::from(51));

    }


}