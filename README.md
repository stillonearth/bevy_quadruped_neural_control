# Bevy Quadruped Neural Control

Control quadruped robot in simulation using neural network. These demos use `bevy_mujoco` (mujoco physics for bevy) and `tract` (invoke neural networks in Rust) to make an environment with neurally-controlled quadruped robot Unitree A1.

https://user-images.githubusercontent.com/97428129/205448884-15360ecf-ff24-4dfd-a27c-899184dc70a1.mp4

Details on control system synthesis [here](https://github.com/stillonearth/continuous_control-unitree-a1).

## Running Neural Network in Rust Environment with Sonos Tract

```bash
cargo run --example a1_walk
```
