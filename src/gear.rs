use bevy::prelude::*;

#[derive(Component)]
pub struct Gear {

}

impl Gear {
    pub fn spawn(mut commands: Commands, assets: Res<super::Assets>) {
        commands.spawn_bundle(SpriteBundle {
            texture: assets.gear.clone(),
            ..Default::default()
        }).insert(Gear{});
    }

    pub fn update() {

    }
}