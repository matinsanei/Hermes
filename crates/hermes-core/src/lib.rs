#![no_std]

extern crate alloc;
use alloc::vec::Vec;

/// Represents the state of the robot at a specific point in time.
/// This is a generic container that can hold various sensor readings.
#[derive(Debug, Clone, Default)]
pub struct RobotState {
    pub timestamp: u64, // Microseconds since boot
    // We can add a generic map or specific fields here. 
    // For now, let's keep it simple with a vector of float values for sensors.
    pub sensors: Vec<f32>, 
}

/// Interface for interacting with the hardware (sensors and actuators).
/// This trait will be implemented by the platform-specific code (e.g., hermes-node on ESP32, hermes-sim on PC).
pub trait HardwareInterface {
    /// Reads the current state of the sensors.
    fn read_sensors(&mut self) -> Result<RobotState, HardwareError>;

    /// Writes commands to the actuators.
    /// `commands` is a vector of values corresponding to actuators.
    fn write_actuators(&mut self, commands: &[f32]) -> Result<(), HardwareError>;
}

#[derive(Debug)]
pub enum HardwareError {
    SensorReadFailed,
    ActuatorWriteFailed,
    CommunicationError,
}

/// The brain/reflex logic.
/// Takes the current state and decides what to do.
pub trait Controller {
    fn update(&mut self, state: &RobotState) -> Vec<f32>;
}
