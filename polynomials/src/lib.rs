
pub mod univariat_polynomial;
trait Polynomial {

    fn evaluate(&self, x: f64) -> f64;

    fn add(&self, other: &Self) -> Self;

    fn multiply(&self, other: &Self) -> Self;

    fn poly_length(&self) -> usize;

}