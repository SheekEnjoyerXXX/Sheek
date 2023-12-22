use bevy::prelude::*;

use crate::prelude::*;

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                toggle_dev,
                dev_move.run_if(in_state(AppState::Dev)),
            ),
        );
    }
}

fn toggle_dev(
    keys: Res<Input<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Semicolon) {
        if state.get() == &AppState::Dev {
            next_state.set(AppState::Gaming);
        } else if state.get() == &AppState::Gaming {
            next_state.set(AppState::Dev);
        }
    }
}

fn dev_move(
    mut q_camera: Query<&mut Transform, (With<PlayerCamera>, With<Camera>)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Some(mut camera) = q_camera.iter_mut().next() {
        let up = keys.any_pressed([KeyCode::W, KeyCode::Up]);
        let down = keys.any_pressed([KeyCode::S, KeyCode::Down]);
        let left = keys.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keys.any_pressed([KeyCode::D, KeyCode::Right]);

        let mut to_move = Vec3::ZERO;

        if up {
            to_move.y += 1.;
        }
        if down {
            to_move.y -= 1.;
        }

        if left {
            to_move.x -= 1.;
        }
        if right {
            to_move.x += 1.;
        }

        let speed = if keys.pressed(KeyCode::ShiftLeft) {
            4.
        } else {
            1.
        };

        camera.translation += to_move * 200. * speed * time.delta_seconds();
    }
}