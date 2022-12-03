use bevy::prelude::*;
use bevy_flycam::*;
use bevy_inspector_egui::*;
use bevy_mujoco::*;

use rand::Rng;

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

fn robot_control_loop(mut mujoco_resources: ResMut<MuJoCoResources>) {
    println!("Sensor reading: {:?}", mujoco_resources.state);

    let mut rng = rand::thread_rng();

    let mut control: Vec<f64> = vec![0.0; mujoco_resources.control.number_of_controls];
    for i in 0..mujoco_resources.control.number_of_controls {
        control[i] = rng.gen::<f64>();
    }

    mujoco_resources.control.data = control;
}

fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(MuJoCoPluginSettings {
            // model_xml_path: "assets/mjcf/simple_4.xml".to_string(),
            model_xml_path: "assets/unitree_a1/scene.xml".to_string(),
            model_assets_path: "assets/unitree_a1/assets/".to_string(),
            pause_simulation: false,
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(NoCameraPlayerPlugin)
        // .insert_resource(MovementSettings {
        //     speed: 3.0,
        //     ..default()
        // })
        .add_plugin(MuJoCoPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(robot_control_loop)
        .run();
}
