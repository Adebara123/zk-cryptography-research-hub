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