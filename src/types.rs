pub mod point;
pub mod vector;

pub trait HasLength {
    fn length(&self) -> f64;
}
