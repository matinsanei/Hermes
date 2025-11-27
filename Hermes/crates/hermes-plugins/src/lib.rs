//! Hermes Plugins
//!
//! A collection of small controller building blocks. Each module should be kept small and testable.

/// A tiny PID controller example
#[derive(Default, Debug)]
pub struct Pid {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    pub integral: f64,
    pub last_error: f64,
}

impl Pid {
    pub fn next(&mut self, error: f64, dt: f64) -> f64 {
        self.integral += error * dt;
        let deriv = (error - self.last_error) / dt.max(1e-12);
        self.last_error = error;
        self.kp * error + self.ki * self.integral + self.kd * deriv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pid_basic() {
        let mut p = Pid { kp: 1.0, ki: 0.0, kd: 0.0, integral: 0.0, last_error: 0.0 };
        let out = p.next(2.0, 0.1);
        assert_eq!(out, 2.0);
    }
}
