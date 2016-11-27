use Cost;
use Neighbor;

extern crate rand;
use annealing::rand::Rng;

pub trait Schedule {
  fn temp(step: u32, step_max: u32) -> f64;
}

pub struct Annealer<T: Cost + Neighbor> {
  state: T,
  energy: f64
}

impl<T> Annealer<T> where T: Cost + Neighbor {
  pub fn new(start: T) -> Annealer<T> {
    let energy = start.cost();
    Annealer {
      state: start,
      energy: energy
    }
  }

  pub fn optimize<S: Schedule>(&mut self, steps: u32) -> &T {
    let mut visits = 0;
    let mut rng = rand::thread_rng();
    for step in 0..steps {
      let neighbor = self.state.neighbor();
      let neighbor_energy = neighbor.cost();
      let temp = S::temp(step, steps);
      let x : f64 = rng.gen();
      //println!("x={}", x);
      if x < Self::acceptance(self.energy, neighbor_energy, temp) {
        self.state = neighbor;
        self.energy = neighbor_energy;
        visits += 1;
      }

      if step > 0 && step % 200 == 0 {
        println!("Step {}-{}: visited {} states; energy {}", step-200, step, visits, self.energy);
        visits = 0;
      }
    }
    &self.state
  }

  fn acceptance(e_old: f64, e_new: f64, temp: f64) -> f64 {
    if e_new < e_old {
      1.
    } else {
      (-(e_new - e_old) / temp).exp()
    }
  }
}
