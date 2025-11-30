# Hermes


# ⚡ Hermes
> **An Experiment in Embedded Rust: Bridging Real-time Control and AI**

[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Status: Experimental](https://img.shields.io/badge/Status-Experimental-yellow.svg)]()

**Hermes is not a product; it is a question.**

We are trying to find out if modern system languages (Rust) can unlock high-end robotics performance on low-cost hardware. We are building a middleware that attempts to run nanosecond-level predictive control loops on chips like ESP32, while accepting high-level commands from an external AI.

**We are not sure if this architecture will succeed, but we believe the path is worth exploring.**

---

## ❓ The Questions We Are Asking

This project exists to find answers to specific engineering bottlenecks in modern robotics:

1.  **Can we decouple "Reflex" from "Intent"?**
    Current robots keep the AI and the control loop tight. This causes latency issues. We want to treat the microcontroller (ESP32) as a human **spinal cord** (handling involuntary balance reflexes locally) and the PC as the **brain** (handling decision making). Can we separate them without crashing?

2.  **Is bare-metal Rust fast enough to predict the future?**
    Standard microcontrollers struggle with heavy predictive math (MPC/LQR). We assume that Rust's zero-cost abstractions might allow us to run complex "error prediction" models on cheap hardware within nanoseconds, effectively creating a "cheating" controller that reacts *before* an error occurs.

3.  **Can we make firmware dynamic?**
    Why do we need to re-flash the chip just to tweak a PID formula? We are trying to build a tiny math interpreter that lets us hot-swap control logic over a serial connection instantly.

---

## ⚠️ The Reality Check (Disclaimer)

**This project might fail.**

We are pushing 2-dollar chips to their limits. We might face:
*   Hardware memory constraints (RAM exhaustion).
*   Latency issues in the communication protocol that destabilize the robot.
*   The realization that complex kinematic models are simply too heavy for `no_std` implementation.

If this happens, we document the failure, learn from it, and pivot. If it succeeds, however, it democratizes high-end robotics control.

---

## 🗺️ The Vision (Roadmap)

We are currently navigating through this fog:

*   **First Goal:** Create a modular "Math Engine" in Rust that can execute control logic string formulas dynamically, independent of hardware.
*   **Second Goal:** Validate the logic in a PC-based simulation (Mock Physics). If we can't balance a virtual pendulum here, we won't touch the hardware.
*   **Third Goal:** Port the engine to ESP32 using the `embassy` async framework and attempt to balance a simple inverted pendulum.
*   **The Horizon:** Establish a bi-directional link where the ESP32 keeps the robot standing (Survival Mode) while an external python script sends movement directives (Voluntary Mode).

---

*"Science is not about knowing the answers, it is about asking the right questions."*



















Hermes is a modular robotics framework aiming to provide a clean separation between core algorithms, hardware-specific targets, and developer tools.

Repository layout

```
Hermes/
├── Cargo.toml                  # Workspace Manager
├── README.md
│
├── crates/                     # core non-hardware crates
│   ├── hermes-core/            # 🧠 core traits and state management
│   ├── hermes-math/            # 🧮 math & dynamic formula parser
│   ├── hermes-protocol/        # 📡 communication protocol shared PC <-> ESP
│   └── hermes-plugins/         # 🧩 plugins (PID, LQR, Kalman...)
│
├── targets/                    # platform-specific apps
│   ├── hermes-node/            # 🤖 ESP32 firmware
│   └── hermes-sim/             # 🎮 PC simulator
│
└── tools/                      # developer tools
    └── hermes-cli/             # ⌨️ command-line controller
```

Quick start

1. Install Rust and Cargo: https://rust-lang.org
2. From repo root run `cargo build --workspace`

---
Licensed under MIT 


