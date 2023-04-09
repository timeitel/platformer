mod gameplay;
// use game::GamePlugin;
// use main_menu::MainMenuPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // .add_plugin(MainMenuPlugin)
        // .add_plugin(GamePlugin)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Gameplay,
    GameOver,
}
