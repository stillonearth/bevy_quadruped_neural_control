# Bevy Quadruped Neural Control

[![Crates.io](https://img.shields.io/crates/v/bevy_quadruped_neural_control.svg)](https://crates.io/crates/bevy_quadruped_neural_control)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/bevyengine/bevy#license)
[![Rust](https://github.com/stillonearth/bevy_quadruped_neural_control/workflows/CI/badge.svg)](https://github.com/stillonearth/bevy_quadruped_neural_control/actions)

##

Control quadruped robot in simulation using neural network. These demos use `bevy_mujoco` (mujoco physics for bevy) and `tract` (invoke neural networks in Rust) to make an environment with neurally-controlled quadruped robot Unitree A1.

https://user-images.githubusercontent.com/97428129/210613348-82a5e59d-96af-42a9-a94a-c47093eb8297.mp4

Details on control system synthesis [here](https://github.com/stillonearth/continuous_control-unitree-a1).

## Running Neural Network in Rust Environment with Sonos Tract

```bash
cargo run --example a1_walk
```
