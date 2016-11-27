pub trait Cost {
  fn cost(&self) -> f64;
}
pub trait Neighbor {
  fn neighbor(&self) -> Self;
}

//pub struct Annealer<T: Cost + Neighbor> {
//}

pub mod annealing;
