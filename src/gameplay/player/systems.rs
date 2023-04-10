use crate::gameplay::animation::components::*;

use super::components::Player;
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let atlas = TextureAtlas::from_grid(
        asset_server.load("Main Characters/Mask Dude/Idle (32x32).png"),
        Vec2::splat(32.),
        11,
        1,
        None,
        None,
    );

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas.add(atlas),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        },
        Player,
        SpriteAnimation {
            len: 11,
            frame_time: 1. / 20.,
        },
        FrameTime(0.0),
    ));
}
