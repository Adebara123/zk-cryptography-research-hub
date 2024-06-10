
use ark_ff::Field;
use ark_ff::PrimeField;


#[derive(Clone, Debug, PartialEq)]
pub struct UnivariantPolynomial<F: Field> {
    coefficients: Vec<F>,
}

impl <F: PrimeField> UnivariantPolynomial<F> {
    
    pub fn new(coefficient: Vec<F>) -> Self{

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


    pub fn mul(&self, other: Self) -> Self {
        // If either polynomial is zero, return a zero polynomial
        if self.check_zero_len() || other.check_zero_len() {
            return Self::new(Vec::new());
        }

        // Degree of the resulting polynomial
        let first_degree = self.coefficient_len();
        let second_degree = other.coefficient_len();
        let new_highest_degree = first_degree + second_degree - 2; // -2 because degrees are length - 1

        // Initialize the coefficients of the resulting polynomial with zeros
        let mut coefficient_res: Vec<F> = vec![F::zero(); new_highest_degree + 1];

        // Compute the coefficients of the resulting polynomial
        for i in 0..first_degree {
            for j in 0..second_degree {
                coefficient_res[i + j] += self.coefficients[i] * other.coefficients[j];
            }
        }

        // Return the resulting polynomial
        Self::new(coefficient_res)
    
}


pub fn evaluate(&self, eval: F) -> F {
    // Initialize the result as zero
    let mut res = F::zero();

    // Iterate over the coefficients in reverse order to use Horner's method
    for &coeff in self.coefficients.iter().rev() {
        res = res * eval + coeff;
    }

    res
}



}


/// A vector containing the coefficients of the Lagrange basis polynomial \( L_i(x) \).
fn lagrange_basis<F: PrimeField>(i: usize, x_coordinates: &[F]) -> Vec<F> {
    // Get the \( i \)-th \( x \) coordinate
    let x_i = x_coordinates[i];
    // Initialize the numerator polynomial with 1
    let mut num = vec![F::one()];

    // Iterate over all other \( x \) coordinates to construct the basis polynomial
    for (j, &x_j) in x_coordinates.iter().enumerate() {
        // Skip if \( j = i \)
        if i != j {
            // Construct polynomial (x - x_j)
            let mut new_num = vec![F::zero(); num.len() + 1]; // Allocate space for degree n-1
            for k in 0..num.len() {
                new_num[k + 1] += num[k]; // Increase the degree of each term by 1
                new_num[k] -= num[k] * x_j; // Multiply each term by (x - x_j)
            }
            num = new_num; // Update the numerator polynomial
        }
    }

    // Compute the denominator of the basis polynomial
    let mut den = F::one();
    for (j, &x_j) in x_coordinates.iter().enumerate() {
        if i != j {
            den *= x_i - x_j;
        }
    }

    // Compute the inverse of the denominator
    let den_inv = den.inverse().unwrap();
    // Normalize the coefficients of the numerator polynomial by dividing by the denominator
    for coeff in num.iter_mut() {
        *coeff *= den_inv;
    }

    // Return the coefficients of the Lagrange basis polynomial
    num
}


/// A vector containing the coefficients of the Lagrange interpolating polynomial.
pub fn lagrange_interpolate<F: PrimeField>(x_coordinates: &[F], y_coordinates: &[F]) -> Vec<F> {
    // Get the number of points
    let n = x_coordinates.len();
    // Initialize the result vector with zeros
    let mut result = vec![F::zero(); n];

    // Iterate over each \( y \) coordinate
    for (i, &y_i) in y_coordinates.iter().enumerate() {
        // Compute the Lagrange basis polynomial for the \( i \)-th point
        let l_i = lagrange_basis(i, x_coordinates);
        // Accumulate the scaled basis polynomial into the result
        for (j, &coeff) in l_i.iter().enumerate() {
            result[j] += y_i * coeff;
        }
    }

    // Return the coefficients of the Lagrange interpolating polynomial
    result
}

#[cfg(test)]
mod tests {

 
    use crate::univariate_poly::univariat_polynomial::lagrange_interpolate;

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

    #[test]
    fn test_polynomial_interpolation() {
        
        let x_coordinates: Vec<F> = vec![
            F::from(1u64),
            F::from(2u64),
            F::from(3u64),
        ];
    
        let y_coordinates: Vec<F> = vec![
            F::from(6u64),
            F::from(17u64),
            F::from(34u64),
        ];
    
        let expected_coefficients = vec![F::from(1u64), F::from(2u64), F::from(3u64)];
        // let coefficients = lagrange_interpolate(&x_coordinates, &y_coordinates);
        // assert_eq!(coefficients, expected_coefficients);
        
        assert_eq!(lagrange_interpolate(&x_coordinates, &y_coordinates), expected_coefficients)
    }


    
}
