mod gameplay;

use bevy::prelude::*;
use gameplay::player::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // .add_plugin(MainMenuPlugin)
        // .add_plugin(GamePlugin)
        .add_startup_system(spawn_player)
        .add_system(move_player)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Gameplay,
    GameOver,
}
