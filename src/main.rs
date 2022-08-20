use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::gear::Gear;

mod gear;
mod map;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Next,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Next)
                .with_collection::<Assets>(),
        )
        .add_state(GameState::Loading)
        .insert_resource(Msaa { samples: 1 })
        .add_system_set(SystemSet::on_enter(GameState::Next).with_system(init).with_system(Gear::spawn))
        .run();
}

#[derive(AssetCollection)]
pub struct Assets {
    #[asset(path = "placeholdergear.png")]
    gear: Handle<Image>,
}

pub fn init(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}