fn spawn_player(
    mut commands: Commands,
    mut texture_atlas: ResMut<Asset<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let atlas = TextureAtlas::from_grid(
        assets_server.load("Main Characters/Mask Dude/Idle (32x32).png"),
        Vec2::splat(32.),
        11,
        1,
        None,
        None,
    );

    commands.spawn(
        (SpriteSheetBundle {
            texture_atlas: texture_atlas.add(atlas),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default(),
            },
            ..Default::default()
        }, Player));
}
