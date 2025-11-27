//! Hermes Protocol
//!
//! Defines wire messages and small framing helpers used by PC and embedded targets.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ControlCommand {
    Ping,
    Reset,
    SetFormula(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Telemetry {
    pub timestamp_ms: u64,
    pub values: Vec<f64>,
}

pub fn frame_json<T: serde::Serialize>(t: &T) -> String {
    serde_json::to_string(t).unwrap_or_else(|_| String::from("{}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_frame() {
        let cmd = ControlCommand::Ping;
        let json = frame_json(&cmd);
        assert!(json.contains("Ping"));
    }
}
