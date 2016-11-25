extern crate life;
use life::pattern_finder::Pattern;

extern crate optimizer;
use optimizer::Cost;

fn mean(arr: &[f64]) -> f64 {
  arr.iter().sum::<f64>() / arr.len() as f64
}

fn stddev(arr: &[f64], mu: f64) -> f64 {
  (arr.iter().map(|x| (x - mu) * (x - mu)).sum::<f64>() / arr.len() as f64).sqrt()
}

fn main() {
  let costs: Vec<_> = (0..50).map(|_| Pattern::random().cost()).collect();
  let min = costs.iter().cloned().fold(1., f64::min);
  let max = costs.iter().cloned().fold(0., f64::max);
  let avg = mean(costs.as_slice());
  let sd  = stddev(costs.as_slice(), avg);
  println!("avg: min={} max={} µ={} sd={}", min, max, avg, sd);
}
