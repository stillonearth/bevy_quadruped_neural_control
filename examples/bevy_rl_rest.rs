use bevy::prelude::*;
use bevy_flycam::*;
use bevy_mujoco::*;

use bevy_rl::{render::AIGymPlugin, state::AIGymState, AIGymSettings};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Resource)]
enum AppState {
    InGame,
    Control,
}

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

#[allow(unused_must_use)]
fn pause_simulation(
    mut app_state: ResMut<State<AppState>>,
    ai_gym_state: Res<AIGymState<Actions, EnvironmentState>>,
    mut mujoco_settings: ResMut<MuJoCoPluginSettings>,
    ai_gym_settings: Res<AIGymSettings>,
    mujoco_resources: Res<MuJoCoResources>,
) {
    if *app_state.current() == AppState::Control {
        return;
    }

    app_state.push(AppState::Control);
    mujoco_settings.pause_simulation = true;

    let mut ai_gym_state = ai_gym_state.lock().unwrap();
    let results = (0..ai_gym_settings.num_agents).map(|_| true).collect();
    ai_gym_state.send_step_result(results);

    let env_state = EnvironmentState(mujoco_resources.state.clone());
    ai_gym_state.set_env_state(env_state);
}

pub(crate) fn process_control_request(
    ai_gym_state: ResMut<AIGymState<Actions, EnvironmentState>>,
    mut app_state: ResMut<State<AppState>>,
    mut mujoco_settings: ResMut<MuJoCoPluginSettings>,
    mut mujoco_resources: ResMut<MuJoCoResources>,
) {
    let ai_gym_state = ai_gym_state.lock().unwrap();

    // Drop the system if users hasn't sent request this frame
    if !ai_gym_state.is_next_action() {
        return;
    }

    let unparsed_actions = ai_gym_state.receive_action_strings();

    for i in 0..unparsed_actions.len() {
        if let Some(unparsed_action) = unparsed_actions[i].clone() {
            let action: Vec<f64> = serde_json::from_str(&unparsed_action).unwrap();
            mujoco_resources.control.data = action;
        }
    }

    mujoco_settings.pause_simulation = false;

    app_state.pop().unwrap();
}

#[derive(Resource)]
struct DelayedControlTimer(Timer);

fn main() {
    let gym_settings = AIGymSettings {
        num_agents: 1,
        render_to_buffer: false,
        ..default()
    };

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
    app.add_state(AppState::InGame);
    app.insert_resource(gym_settings.clone())
        .insert_resource(AIGymState::<Actions, EnvironmentState>::new(
            gym_settings.clone(),
        ))
        .add_plugin(AIGymPlugin::<Actions, EnvironmentState>::default());

    app.add_system(pause_simulation.after("mujoco_simulate"));
    app.add_system_set(
        SystemSet::on_update(AppState::Control).with_system(process_control_request),
    );

    // Start
    app.run();
}
