extern crate life;
use life::pattern_finder::Pattern;

extern crate optimizer;
use optimizer::Cost;
use optimizer::annealing::{Annealer, Schedule};

fn mean(arr: &[f64]) -> f64 {
  arr.iter().sum::<f64>() / arr.len() as f64
}

fn stddev(arr: &[f64], mu: f64) -> f64 {
  (arr.iter().map(|x| (x - mu) * (x - mu)).sum::<f64>() / arr.len() as f64).sqrt()
}

struct BasicSchedule;
impl Schedule for BasicSchedule {
  fn temp(step: u32, step_max: u32) -> f64 {
    0.0005 / (1. + (1. + step as f64).ln())
  }
}

fn main() {
  let start = Pattern::random();
  let start_cost = start.cost();
  let mut annealer = Annealer::<Pattern>::new(start);
  let end_cost = annealer.optimize::<BasicSchedule>(5000).cost();
  println!("Start cost: {}", start_cost);
  println!("End cost:   {}", end_cost);
  println!("Improvement: {}", -(end_cost-start_cost));
}
