use std::time::Duration;

use bevy::{prelude::*, asset::ChangeWatcher, winit::{WinitSettings, UpdateMode}};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};

mod input;
mod camera;
mod entities;
mod dev;

pub mod prelude {
    pub use super::{input::*, camera::*, entities::*, dev::*};
    pub mod globals {
        pub const ASPECT_RATIO: (u8, u8) = (16, 10);
        pub const WIDTH: f32 = 320.*1.25;
        pub const HEIGHT: f32 = 200.*1.25;
        pub const TARGET_RATIO: f32 = ASPECT_RATIO.0 as f32 / ASPECT_RATIO.1 as f32;
    }
    pub use super::AppState;
}

use bevy_ecs_tilemap::map::TilemapRenderSettings;
use prelude::*;

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum AppState {
    MainMenu,
    #[default]
    Gaming,
    LevelTransition,
    Frozen,
    Paused,
    Dev,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "sheeek".into(),
                        resolution: bevy::window::WindowResolution::new(
                            globals::WIDTH * 4.0,
                            globals::HEIGHT * 4.0,
                        ),
                        resize_constraints: WindowResizeConstraints {
                            min_width: globals::WIDTH * 2.,
                            min_height: globals::HEIGHT * 2.,
                            ..default()
                        },
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    asset_folder: "assets".into(),
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                })
                .build(),
        )
        .insert_resource(Msaa::Off)
        .insert_resource(WinitSettings {
            unfocused_mode: UpdateMode::ReactiveLowPower {
                max_wait: Duration::from_secs(60),
            },
            ..default()
        })
        .insert_resource(TilemapRenderSettings {
            render_chunk_size: UVec2::new(32, 32),
            y_sort: false,
            ..default()
        })
        .add_state::<AppState>()
        .insert_resource(LevelSelection::Index(0))
        .add_plugins((LdtkPlugin, PlayerPlugin, PlayerCameraPlugin, EntitiesPlugin, DevPlugin))
        .add_systems(Startup, setup)
        .run()
    ;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels/yes.ldtk"),
        ..Default::default()
    });
}
