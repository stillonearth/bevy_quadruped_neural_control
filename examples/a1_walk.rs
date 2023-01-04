#![feature(allocator_api)]

use std::alloc::Global;

use bevy::prelude::*;
use bevy_flycam::*;
use bevy_mujoco::*;

use tract_ndarray::Array2;
use tract_onnx::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    static ref MODEL: SimplePlan<TypedFact, Box<dyn TypedOp, Global>, Graph<TypedFact, Box<dyn TypedOp, Global>>> =
        tract_onnx::onnx()
            .model_for_path("assets/g1-forward.onnx")
            .unwrap()
            .with_input_fact(0, f32::fact([1, 119]).into())
            .unwrap()
            .into_optimized()
            .unwrap()
            .into_runnable()
            .unwrap();
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(FlyCam);
}

// These numbers aren't exported to onnx, it's action scaling coefficients from original python code
// https://github.com/DLR-RM/stable-baselines3/blob/4fa17dcf0f72455aa3d36308291d4b052b2544f7/stable_baselines3/common/policies.py#L263
const LOW: [f32; 12] = [
    -0.802851, -1.0472, -2.69653, -0.802851, -1.0472, -2.69653, -0.802851, -1.0472, -2.69653,
    -0.802851, -1.0472, -2.69653,
];

const HIGH: [f32; 12] = [
    0.802851, 4.18879, -0.916298, 0.802851, 4.18879, -0.916298, 0.802851, 4.18879, -0.916298,
    0.802851, 4.18879, -0.916298,
];

fn robot_control_loop(mut mujoco_resources: ResMut<MuJoCoResources>) {
    // prepare simulation data for the NN
    let qpos = mujoco_resources.state.qpos.clone();
    let qvel = mujoco_resources.state.qvel.clone();
    let cfrc_ext = mujoco_resources.state.cfrc_ext.clone();

    // make an input vector for a neural network
    let mut input_vec: Vec<f32> = Vec::new();
    for i in 2..qpos.len() {
        input_vec.push(qpos[i] as f32);
    }
    for i in 0..qvel.len() {
        input_vec.push(qvel[i] as f32);
    }
    for i in 0..cfrc_ext.len() {
        input_vec.push(cfrc_ext[i][0] as f32);
        input_vec.push(cfrc_ext[i][1] as f32);
        input_vec.push(cfrc_ext[i][2] as f32);
        input_vec.push(cfrc_ext[i][3] as f32);
        input_vec.push(cfrc_ext[i][4] as f32);
        input_vec.push(cfrc_ext[i][5] as f32);
    }

    // convert this to a tensor
    let input: Tensor = Array2::from_shape_vec((1, 119), input_vec).unwrap().into();
    // get model execution result
    let result = MODEL.run(tvec!(input.into())).unwrap();
    // extract model output
    let actions = result[0].to_array_view::<f32>().unwrap();
    // prepare control vector fo passing to mujoco
    let mut control: Vec<f64> = vec![0.0; mujoco_resources.control.number_of_controls];
    // fill control vector with actions (copy and adjust model output)
    for i in 0..mujoco_resources.control.number_of_controls {
        control[i] = actions[[0, i]] as f64;
        // scaling actions
        control[i] = LOW[i] as f64 + 0.5 * (control[i] + 1.0) * (HIGH[i] as f64 - LOW[i] as f64);
    }

    mujoco_resources.control.data = control;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MuJoCoPluginSettings {
            model_xml_path: "assets/unitree_a1/scene.xml".to_string(),
            pause_simulation: false,
            target_fps: 600.0, // this is not actual fps (bug in bevy_mujoco),
                               // the bigger the value, the slower the simulation
        })
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            speed: 1.0,
            ..default()
        })
        .add_plugin(MuJoCoPlugin)
        .add_startup_system(setup)
        .add_system(robot_control_loop.after("mujoco_simulate"))
        .run();
}
