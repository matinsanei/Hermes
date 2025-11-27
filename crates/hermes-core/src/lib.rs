//! Hermes Core
//!
//! This crate contains core traits and basic state-management primitives used across targets and plugins.
//!
//! Example trait:

pub trait StateManager {
    /// Advances the internal state by one step.
    fn step(&mut self, dt_secs: f32);

    /// Returns a debug snapshot of the internal state
    fn snapshot(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Dummy {
        t: f32,
    }

    impl StateManager for Dummy {
        fn step(&mut self, dt_secs: f32) { self.t += dt_secs; }
        fn snapshot(&self) -> String { format!("t={:.3}", self.t) }
    }

    #[test]
    fn dummy_steps() {
        let mut d = Dummy { t: 0.0 };
        d.step(0.1);
        assert_eq!(d.snapshot(), "t=0.100");
    }
}
