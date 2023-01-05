// This example shows how to use the REST API to control the simulation
// It uses the bevy_rl crate to provide a REST API

use bevy::prelude::*;
use bevy_flycam::*;
use bevy_mujoco::*;

use bevy_rl::{
    state::AIGymState, AIGymPlugin, AIGymSettings, EventControl, EventPauseResume, SimulationState,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Deref, DerefMut, Clone, Deserialize)]
pub struct Actions(Vec<f64>);

// Observation space

#[derive(Default, Deref, DerefMut, Serialize, Clone)]
pub struct EnvironmentState(MuJoCoState);

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

fn bevy_rl_pause_request(
    mut pause_event_reader: EventReader<EventPauseResume>,
    mut mujoco_settings: ResMut<MuJoCoPluginSettings>,
    mujoco_resources: Res<MuJoCoResources>,
    ai_gym_state: Res<AIGymState<Actions, EnvironmentState>>,
) {
    for _ in pause_event_reader.iter() {
        // Pause physics engine
        mujoco_settings.pause_simulation = true;
        // Collect state into serializable struct
        let env_state = EnvironmentState(mujoco_resources.state.clone());
        // Set bevy_rl gym state
        let mut ai_gym_state = ai_gym_state.lock().unwrap();
        ai_gym_state.set_env_state(env_state);
    }
}

#[allow(unused_must_use)]
fn bevy_rl_control_request(
    mut pause_event_reader: EventReader<EventControl>,
    mut mujoco_settings: ResMut<MuJoCoPluginSettings>,
    mut mujoco_resources: ResMut<MuJoCoResources>,
    mut simulation_state: ResMut<State<SimulationState>>,
) {
    for control in pause_event_reader.iter() {
        println!("Control request received");
        let unparsed_actions = &control.0;
        for i in 0..unparsed_actions.len() {
            if let Some(unparsed_action) = unparsed_actions[i].clone() {
                let action: Vec<f64> = serde_json::from_str(&unparsed_action).unwrap();
                mujoco_resources.control.data = action;
            }
        }
        // Resume simulation
        mujoco_settings.pause_simulation = false;
        simulation_state.set(SimulationState::Running);
    }
}

fn main() {
    let mut app = App::new();

    // Basic bevy setup
    app.add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            speed: 3.0,
            ..default()
        })
        .add_startup_system(setup);

    // Setup bevy_mujoco
    app.insert_resource(MuJoCoPluginSettings {
        model_xml_path: "assets/unitree_a1/scene.xml".to_string(),
        pause_simulation: false,
        target_fps: 600.0,
    })
    .add_plugin(MuJoCoPlugin);

    // Setup bevy_rl
    let ai_gym_state = AIGymState::<Actions, EnvironmentState>::new(AIGymSettings {
        num_agents: 1,
        render_to_buffer: false,
        pause_interval: 0.01,
        ..default()
    });
    app.insert_resource(ai_gym_state)
        .add_plugin(AIGymPlugin::<Actions, EnvironmentState>::default());

    // bevy_rl events
    app.add_system(bevy_rl_pause_request);
    app.add_system(bevy_rl_control_request);

    // Start
    app.run();
}
