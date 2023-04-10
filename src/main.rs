mod gameplay;
// use game::GamePlugin;
// use main_menu::MainMenuPlugin;

use bevy::prelude::*;
use gameplay::animation::systems::*;
use gameplay::player::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // .add_plugin(MainMenuPlugin)
        // .add_plugin(GamePlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(animate_sprite)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Gameplay,
    GameOver,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
