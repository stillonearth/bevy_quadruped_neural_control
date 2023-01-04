# Bevy Quadruped Neural Control

Control quadruped robot in simulation using neural network. These demos use `bevy_mujoco` (mujoco physics for bevy) and `tract` (invoke neural networks in Rust) to make an environment with neurally-controlled quadruped robot Unitree A1.

https://user-images.githubusercontent.com/97428129/210613348-82a5e59d-96af-42a9-a94a-c47093eb8297.mp4

Details on control system synthesis [here](https://github.com/stillonearth/continuous_control-unitree-a1).

## Running Neural Network in Rust Environment with Sonos Tract

```bash
cargo run --example a1_walk
```
