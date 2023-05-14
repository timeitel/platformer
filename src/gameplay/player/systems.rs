use super::components::Player;
use bevy::prelude::*;

const MOVE_SPEED: f32 = 100.;

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
}

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
