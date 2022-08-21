use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_mod_picking::*;

use crate::gear::Gear;
use crate::map::Map;

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
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(EditorPlugin)
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Next)
                .with_collection::<gear::GearAssets>(),
        )
        .add_state(GameState::Loading)
        .insert_resource(Msaa { samples: 1 })
        .add_system_set(
            SystemSet::on_enter(GameState::Next)
                .with_system(init)
                .with_system(Map::init)
                .with_system(Gear::init)
                .with_system(Gear::spawn),
        )
        .add_system_set(SystemSet::on_update(GameState::Next).with_system(Gear::set_mode))
        .run();
}

pub fn init(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert_bundle(PickingCameraBundle::default());

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 3.0, -5.0),
        ..Default::default()
    });
}
