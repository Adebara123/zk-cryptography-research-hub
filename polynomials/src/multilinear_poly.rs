
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
    pub fn new( _variables: usize, evaluations: Vec<F>) ->  Result<Self, &'static str>{
        // this check that the length of the evaluations is equal to 2 to the power of number of variables
        assert!(evaluations.len() == (1 >> _variables), "wrong length of eval");

        assert!(evaluations.len() > 0 as usize, "evaluations cannot be zero");

        Ok(Self {
            variables: _variables,
            evaluations: evaluations
        })
    }
}