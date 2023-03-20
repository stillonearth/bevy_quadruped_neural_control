# Bevy Quadruped Neural Control

[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/bevyengine/bevy#license)
[![Rust](https://github.com/stillonearth/bevy_quadruped_neural_control/workflows/CI/badge.svg)](https://github.com/stillonearth/bevy_quadruped_neural_control/actions)

Control quadruped robot in simulation using neural network. These demos use [bevy_mujoco](https://github.com/stillonearth/bevy_mujoco) (mujoco physics for bevy) and [sonos/tract](https://github.com/sonos/tract) (invoke neural networks in Rust) to make an environment with neurally-controlled quadruped robot Unitree A1.

https://user-images.githubusercontent.com/97428129/210613348-82a5e59d-96af-42a9-a94a-c47093eb8297.mp4

| Example                                                                                                                              | Description                                                                                                                                                                                                                    |
| ------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [examples/bevy_rl_rest.rs](https://github.com/stillonearth/bevy_quadruped_neural_control/blob/main/examples/bevy_rl_rest.rs)         | runs Unitree A1 simulation wrapped as Reinforcement Learning Gym envronment with [bevy_rl](https://github.com/stillonearth/bevy_rl). It also rust REST API so you can control a robot from another environment such as python. |
| [python/policy.py](https://github.com/stillonearth/bevy_quadruped_neural_control/blob/main/python/policy.py)                         | is an example how to control a robot with trained [stable_baselines3/SAC](https://stable-baselines3.readthedocs.io/en/master/modules/sac.html) policy from python env                                                          |
| [python/run_onnx_policy.ipynb](https://github.com/stillonearth/bevy_quadruped_neural_control/blob/main/python/run_onnx_policy.ipynb) | exports PyTorch stable_baselines3 SAC policy to [Open Neural Network Exchange](https://onnx.ai/) format and runs the policy from python env                                                                                    |
| [examples/a1_walk.rs](https://github.com/stillonearth/bevy_quadruped_neural_control/blob/main/examples/a1_walk.rs)                   | runs Unitree A1 simulation and ONNX neural network to control it all in Rust env                                                                                                                                               |

Details on control system synthesis [here](https://github.com/stillonearth/continuous_control-unitree-a1).

## Running Neural Network in Rust Environment with Sonos Tract

```bash
cargo run --example a1_walk
```

## Running Rust Simulator and Neural Networks in Python

```bash
cargo run --example bevy_rl_rest &
cd python && python policy.py
```

=======

# Bevy Quadruped Neural Control

[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/bevyengine/bevy#license)
[![Rust](https://github.com/stillonearth/bevy_quadruped_neural_control/workflows/CI/badge.svg)](https://github.com/stillonearth/bevy_quadruped_neural_control/actions)

Control quadruped robot in simulation using neural network. These demos use [bevy_mujoco](https://github.com/stillonearth/bevy_mujoco) (mujoco physics for bevy) and [sonos/tract](https://github.com/sonos/tract) (invoke neural networks in Rust) to make an environment with neurally-controlled quadruped robot Unitree A1.

https://user-images.githubusercontent.com/97428129/210613348-82a5e59d-96af-42a9-a94a-c47093eb8297.mp4

| Example                                                                                                                              | Description                                                                                                                                                                                                                    |
| ------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [examples/bevy_rl_rest.rs](https://github.com/stillonearth/bevy_quadruped_neural_control/blob/main/examples/bevy_rl_rest.rs)         | Tuns Unitree A1 simulation wrapped as Reinforcement Learning Gym envronment with [bevy_rl](https://github.com/stillonearth/bevy_rl). It also rust REST API so you can control a robot from another environment such as python. |
| [python/policy.py](https://github.com/stillonearth/bevy_quadruped_neural_control/blob/main/python/policy.py)                         | An example how to control a robot with trained [stable_baselines3/SAC](https://stable-baselines3.readthedocs.io/en/master/modules/sac.html) policy from python env                                                             |
| [python/run_onnx_policy.ipynb](https://github.com/stillonearth/bevy_quadruped_neural_control/blob/main/python/run_onnx_policy.ipynb) | Exports PyTorch stable_baselines3 SAC policy to [Open Neural Network Exchange](https://onnx.ai/) format and runs the policy from python env                                                                                    |
| [examples/a1_walk.rs](https://github.com/stillonearth/bevy_quadruped_neural_control/blob/main/examples/a1_walk.rs)                   | Runs Unitree A1 simulation and ONNX neural network to control it all in Rust env                                                                                                                                               |

Details on control system synthesis [here](https://github.com/stillonearth/continuous_control-unitree-a1).

## Running Neural Network in Rust Environment with Sonos Tract

```bash
cargo run --example a1_walk
```

## Running Rust Simulator and Neural Networks in Python

```bash
cargo run --example bevy_rl_rest &
cd python && python policy.py
```
