use Cost;
use Neighbor;

extern crate rand;
use annealing::rand::Rng;

pub trait Schedule {
  fn temp(step: u32, step_max: u32) -> f64;
}

pub struct Annealer<'a, T: Cost + Neighbor + 'a> {
  state: T,
  energy: f64,
  visit_cb: Box<FnMut(&T) + 'a>
}

fn default_cb<T>(_: &T) {}

impl<'a, T> Annealer<'a, T> where T: Cost + Neighbor {
  pub fn new(start: T) -> Annealer<'a, T> {
    let energy = start.cost();
    Annealer {
      state: start,
      energy: energy,
      visit_cb: Box::new(default_cb::<T>)
    }
  }

  pub fn set_visit_cb(&mut self, cb: Box<FnMut(&T) + 'a>) {
    self.visit_cb = cb;
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
        (self.visit_cb)(&self.state);
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
