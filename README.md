# Bevy Quadruped Neural Control

Control quadruped robot in simulation using neural network. This projects uses bevy_mujoco and bevy_rl to make an environment with neurally-controlled quadruped robot Unitree A1.

https://user-images.githubusercontent.com/97428129/205448884-15360ecf-ff24-4dfd-a27c-899184dc70a1.mp4

For details on control system synthesis: https://github.com/stillonearth/continuous_control-unitree-a1

## Running control

```bash
cargo run --example bevy_rl_rest &
python python/policy.py
```

## Running Neural Network in Rust Environment with Sonos Tract

```bash
cargo run --example tract
```
