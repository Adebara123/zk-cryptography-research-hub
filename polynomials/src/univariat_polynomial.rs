
use ark_ff::Field;
use ark_ff::PrimeField;


#[derive(Clone, Debug, PartialEq)]
pub struct UnivariantPolynomial<F: Field> {
    coefficients: Vec<F>,
}

impl <F: PrimeField> UnivariantPolynomial<F> {
    
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

    pub fn add (&self, other: Self) -> Self{

        // Check if the any of them has zero length coefficient 
        
        if self.check_zero_len() {
            return other 
        }

        if other.check_zero_len() {
            return self.clone();
        }

        if self.check_zero_len() != other.check_zero_len() {
            panic!("Cannot add two polynomials of different length")
        }


            let mut new_coefficient = self.coefficients.clone();
            for i in 0..self.coefficient_len() {

                new_coefficient[i] += other.coefficients[i];
            }

            return Self::new(new_coefficient);
      
        
    }


    pub fn mul (&self, other: Self) -> Self {

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

        while let Some(true) = coefficient_res.last().map(|x| *x == F::zero()) {
            coefficient_res.pop();
        }

        Self::new(coefficient_res)
    }


    pub fn evaluate(self, eval: F) -> F {
        
        if eval == F::zero() {
            return F::zero();
        }

        let mut res = F::zero();
        for i in 0..self.coefficient_len() {
            res += (eval.pow([i as u64])) * self.coefficients[i];
        }

        res

    }

    pub fn interpolate(x_coordinates: &Vec<F>, y_coordinates: &Vec<F>) -> Result<Self, &'static str> {

        if x_coordinates.len() != y_coordinates.len() {
            return Err("Vectors x_coordinates and y_coordinates must have the same length.");
        }
        assert_eq!(x_coordinates.len(), y_coordinates.len(), "Vectors x_coordinates and y_coordinates must have the same length.");
    
        let n = x_coordinates.len();
        let mut coefficients = vec![F::zero(); n];
    
        for i in 0..n {
            let mut li = vec![F::one(); n];
            for j in 0..n {
                if i != j {
                    let xi = x_coordinates[i];
                    let xj = x_coordinates[j];
                    for k in (0..n).rev() {
                        li[k] = if k == 0 {
                            -xj * li[k]
                        } else {
                            li[k] * (xi - xj) + li[k-1]
                        };
                    }
                }
            }
            let yi = y_coordinates[i];
            for k in 0..n {
                coefficients[k] += yi * li[k] / li.iter().rev().fold(F::one(), |acc, &x| acc * x);
            }
        }
    
        Ok(Self::new(coefficients))
    }

    

}

#[cfg(test)]
mod tests {

    use ark_ff::Field;
    use ark_ff::PrimeField;
    use super::UnivariantPolynomial;
    use ark_test_curves::bls12_381::Fr;

    type F = Fr;
    type poly = UnivariantPolynomial<F>;

    #[test]
    fn test_polynomial_addition() {

        let poly_1 = poly::new(vec![F::from(1), F::from(2), F::from(3)]);
        let poly_2 = poly::new(vec![F::from(4), F::from(5), F::from(6)]);

        assert!(poly_1.add(poly_2) == poly::new(vec![F::from(5), F::from(7), F::from(9)]));
    }

    #[test]
    fn test_polynomial_multiplication() {

        let poly_1 = poly::new(vec![F::from(1), F::from(2)]);
        let poly_2 = poly::new(vec![F::from(4), F::from(5)]);
        
        assert_eq!(poly_1.mul(poly_2) , poly::new(vec![F::from(4), F::from(13), F::from(10)]));
    }

    #[test]
    fn test_polynomial_evaluation() {
        
        let poly = poly::new(vec![F::from(1), F::from(2), F::from(3)]);
        assert_eq!(poly.evaluate(F::from(10)), F::from(321));
    }


    
}
