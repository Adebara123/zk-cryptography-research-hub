use ark_ff::PrimeField;
use rand::{rngs, thread_rng};
use polynomials::univariat_polynomial;


pub fn create_shares<F: PrimeField>(seceret: F, threshold: u64, members: u64) ->  (Vec<F>, Vec<F>){

    // this is the polynomial created as the seceret
    let mut new_seceret = vec![seceret];

    let mut the_rng = rand::thread_rng();

    for _ in 0..threshold {
        new_seceret.push(F::rand(&mut the_rng));
    }

    // This is the new secret
    let poly_sec = univariat_polynomial::UnivariantPolynomial::new(new_seceret);

    // Creating the coordinate for the secret

    let mut x_coord: Vec<F> = Vec::with_capacity(members as usize);
    let mut y_coord: Vec<F>  = Vec::with_capacity(members as usize);

    for i in 0..members {

        let x_coord_values = F::from(i as u64);
        x_coord.push(x_coord_values);

        let y_coord_values = poly_sec.clone().evaluate(x_coord_values);
        y_coord.push(y_coord_values);
    }


    (x_coord, y_coord)


}

// The parameters would be the x and y coordinates for the secerets 
pub fn construct_seceret<F: PrimeField>(x_coords_sec: Vec<F>, y_coords_sec: Vec<F>, eval_point: F) -> F {

    let the_seceret = univariat_polynomial::lagrange_interpolate(&x_coords_sec, &y_coords_sec);

    let evaluation = univariat_polynomial::UnivariantPolynomial::new(the_seceret);

    // The secret is constructed back, at this point it can be evaluated at zero to remove all cooefficient remaining the constant 
    evaluation.evaluate(eval_point)

}


#[cfg(test)]
mod tests {


    use ark_test_curves::bls12_381::Fr;

    use super::*;

    type F = Fr;
    
    #[test]
    fn test_secret_sharing_creation() {

        let secret = F::from(20);

        let threshold = 4;
        let members = 6;


        let the_shares = create_shares(secret, threshold, members);

        let seceret_construction = construct_seceret(the_shares.0, the_shares.1, Fr::from(0));

        assert_eq!(seceret_construction, F::from(20u64));
    }


}