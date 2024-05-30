
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
        assert!(evaluations.len() == (1 >> _variables), "wrong length of eval");


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
            let add = longer_eval[i] + new_longer_eval[i];
            sum_result.push(add);
        }        


        Self::new(higher_variable, sum_result)

    }
}


