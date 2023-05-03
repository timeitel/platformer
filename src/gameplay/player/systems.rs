use crate::gameplay::animation::components::*;

use super::components::Player;
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let atlas = TextureAtlas::from_grid(
        asset_server.load("sprites/characters/mask_man/Idle (32x32).png"),
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
            frame_time: 1. / 10.,
        },
        FrameTime(0.0),
    ));
}

const MOVE_SPEED: f32 = 100.;

pub fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    let mut transform = player.single_mut();
    if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        transform.translation.x -= MOVE_SPEED * time.delta_seconds();
    } else if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        transform.translation.x += MOVE_SPEED * time.delta_seconds();
    }
}

pub fn change_player_animation(
    mut player: Query<
        (
            &mut Handle<TextureAtlas>,
            &mut SpriteAnimation,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    input: Res<Input<KeyCode>>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let (mut atlas, mut animation, mut sprite) = player.single_mut();

    let left_keys = [KeyCode::Left, KeyCode::A];
    let right_keys = [KeyCode::Right, KeyCode::D];

    if input.any_just_pressed(left_keys) {
        sprite.flip_x = true;
    } else if input.any_just_pressed(right_keys) && !input.any_pressed(left_keys) {
        sprite.flip_x = false;
    } else if input.any_just_released(left_keys)
        && !input.any_pressed(left_keys)
        && input.any_pressed(right_keys)
    {
        sprite.flip_x = false;
    }
}
