use crate::prelude::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Bundle, Default)]
pub struct PlayerEntity {
    sprite_sheet: SpriteSheetBundle,
    player_camera: PlayerCamera,
    player: Player,
    camera_follow: CameraFollow,
}
impl LdtkEntity for PlayerEntity {
    fn bundle_entity(
        _: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let shreek: Handle<Image> = asset_server.load("shreek.png");
        let shreek_atlas = TextureAtlas::from_grid(shreek, Vec2::new(16., 32.), 21, 2, None, None);
        let shreek_handle = texture_atlases.add(shreek_atlas);

        Self {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..default()
                },
                texture_atlas: shreek_handle,
                ..default()
            },
            ..default()
        }
    }
}