
use ark_ff::Field;
use ark_ff::PrimeField;


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
}


#[cfg(test)]
mod tests {


    use crate::multilinear_poly::*;

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



}