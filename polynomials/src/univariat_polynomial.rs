use crate::Polynomial;
use ark_ff::Field;

#[derive(Clone, Debug, PartialEq)]
pub struct UnivariantPolynomial<F: Field> {
    coefficients: Vec<F>,
}

impl <F: Field> UnivariantPolynomial<F> {
    
    fn new(coefficient: Vec<F>) -> Self{

        Self { coefficients: coefficient }
    }

    fn coefficient_len (&self) -> usize {
        self.coefficients.len()
    }

    fn check_zero_len(&self) -> bool {
        if self.coefficient_len() == 0 {
            true
        } else {
            false
        }
    }

    fn add (&self, other: Self) -> Self{

        // Check if the any of them has zero length coefficient 
        
        if self.check_zero_len() {
            return other 
        }

        if other.check_zero_len() {
            return self.clone();
        }

        // Check if both length are equal 
        // [0,1,2,3] [4,2,1,2]

        if self.coefficient_len() == other.coefficient_len() {
            let mut new_coefficient = self.coefficients.clone();
            for i in 0..self.coefficient_len() {

                new_coefficient[i] += other.coefficients[i];
            }

            return Self::new(new_coefficient);
        }

        else {

            // If the first one is longer than the second, slice it and add it up to that point 

            let first_len = self.coefficient_len();
            let second_len = other.coefficient_len() ;
            if first_len > second_len {

                let mut result_coefficient = other.coefficients.clone();

                for i in 0..second_len {
                    result_coefficient[i] += self.coefficients[i];
                }
                let remaining_slice  = &self.coefficients[second_len..first_len-1];

                result_coefficient.extend( remaining_slice);

                return Self::new(result_coefficient);
            } 
            
            else  {
                let mut result_coefficient = self.coefficients.clone();

                for i in 0..first_len {
                    result_coefficient[i] += other.coefficients[i];
                }
                
                let remaining_slice = &other.coefficients[first_len..second_len-1];

                result_coefficient.extend(remaining_slice);

                return Self::new(result_coefficient);
            }
        }
        
    }


    fn mul (&self, other: Self) -> Self {

        // check if any of them is zero 
        if self.check_zero_len() {
            return Self::new(Vec::new()) ;
        }

        if other.check_zero_len() {
            return Self::new(Vec::new()) ;
        }

        let first_degree = self.coefficient_len();
        let second_degree = other.coefficient_len();
        let new_highestdegree = first_degree + second_degree;
        
        let mut coefficient_res: Vec<F> = vec![F::zero(); new_highestdegree + 1];
        
        for i in 0..first_degree {

            for j in 0..second_degree {
                let product = self.coefficients[i] * other.coefficients[j];
                coefficient_res[i + j] += product
            }
        }

        Self::new(coefficient_res)
    }


    fn evaluate(self, eval: F) -> F {
        
        if eval == F::zero() {
            return F::zero();
        }

        let mut res = F::zero();
        for i in 0..self.coefficient_len() {
            res += (eval.pow([i as u64])) * self.coefficients[i];
        }

        res

    }

    fn interpolate () {
        
    }






    

}

#[cfg(test)]
mod tests {


    
}
