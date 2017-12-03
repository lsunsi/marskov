pub struct Brain {
  alpha: f64,
  gamma: f64,
}

impl Brain {
  pub fn new(alpha: f64, gamma: f64) -> Brain {
    Brain {
      alpha: alpha,
      gamma: gamma,
    }
  }
  pub fn learn(&self, current_value: f64, next_value: f64, reward: f64) -> f64 {
    current_value + self.alpha * (reward + self.gamma * next_value - current_value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn alpha_zero_gamma_zero() {
    assert_eq!(
      Brain {
        alpha: 0.,
        gamma: 0.,
      }.learn(1., 2., 3.),
      1.
    );
  }
  #[test]
  fn alpha_zero_gamma_half() {
    assert_eq!(
      Brain {
        alpha: 0.,
        gamma: 0.5,
      }.learn(1., 2., 3.),
      1.
    );
  }
  #[test]
  fn alpha_zero_gamma_one() {
    assert_eq!(
      Brain {
        alpha: 0.,
        gamma: 1.,
      }.learn(1., 2., 3.),
      1.
    );
  }
  #[test]
  fn alpha_half_gamma_zero() {
    assert_eq!(
      Brain {
        alpha: 0.5,
        gamma: 0.,
      }.learn(1., 2., 3.),
      2.
    );
  }
  #[test]
  fn alpha_half_gamma_half() {
    assert_eq!(
      Brain {
        alpha: 0.5,
        gamma: 0.5,
      }.learn(1., 2., 3.),
      2.5
    );
  }
  #[test]
  fn alpha_half_gamma_one() {
    assert_eq!(
      Brain {
        alpha: 0.5,
        gamma: 1.,
      }.learn(1., 2., 3.),
      3.
    );
  }
  #[test]
  fn alpha_one_gamma_zero() {
    assert_eq!(
      Brain {
        alpha: 1.,
        gamma: 0.,
      }.learn(1., 2., 3.),
      3.
    );
  }
  #[test]
  fn alpha_one_gamma_half() {
    assert_eq!(
      Brain {
        alpha: 1.,
        gamma: 0.5,
      }.learn(1., 2., 3.),
      4.
    );
  }
  #[test]
  fn alpha_one_gamma_one() {
    assert_eq!(
      Brain {
        alpha: 1.,
        gamma: 1.,
      }.learn(1., 2., 3.),
      5.
    );
  }
}
