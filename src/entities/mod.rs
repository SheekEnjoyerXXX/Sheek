mod player;
use player::*;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct EntitiesPlugin;
impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerEntity>("Player")
            ;
    }
}