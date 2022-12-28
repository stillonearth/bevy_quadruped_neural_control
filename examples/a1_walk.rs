use bevy::prelude::*;
use bevy_flycam::*;
use bevy_mujoco::*;

use tract_ndarray::Array2;
use tract_onnx::prelude::*;

use rand::Rng;

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
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

fn robot_control_loop(mut mujoco_resources: ResMut<MuJoCoResources>) {
    // load onnx model
    let model = tract_onnx::onnx()
        .model_for_path("assets/g1-control-direction.onnx")
        .unwrap()
        .with_input_fact(0, f32::fact([1, 121]).into())
        .unwrap()
        .into_optimized()
        .unwrap()
        .into_runnable()
        .unwrap();

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
    // control commands
    input_vec.push(3.14); // direction
    input_vec.push(2.65); // orientation

    // for some reasons this might be 2 elements in the beggining of simulation
    if input_vec.len() != 121 {
        return;
    }

    let input: Tensor = Array2::from_shape_vec((1, 121), input_vec).unwrap().into();
    let result = model.run(tvec!(input.into())).unwrap();
    let actions = result[0].to_array_view::<f32>().unwrap();

    let mut control: Vec<f64> = vec![0.0; mujoco_resources.control.number_of_controls];
    for i in 0..mujoco_resources.control.number_of_controls {
        control[i] = actions[[0, i]] as f64;
    }
    mujoco_resources.control.data = control;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MuJoCoPluginSettings {
            model_xml_path: "assets/unitree_a1/scene.xml".to_string(),
            pause_simulation: false,
            target_fps: 1200.0,
        })
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            speed: 1.0,
            ..default()
        })
        .add_plugin(MuJoCoPlugin)
        .add_startup_system(setup)
        .add_system(robot_control_loop)
        .run();
}