use bevy::gltf::{Gltf, GltfMesh};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_mod_picking::{PickableBundle, PickingEvent, PickingCamera};

#[derive(AssetCollection)]
pub struct GearAssets {
    #[asset(path = "gear.glb")]
    gear: Handle<Gltf>,
}

#[derive(Component)]
pub struct Gear {
    mode: Job,
}

pub enum Job {
    Idle,
    Refining,
    Casting,
    Blasting,
    Alloying,
}

pub struct SetModeState {
    drag_direction: Vec2,
    drag_start: Vec2,
    target: Entity,
    original_rot: Quat,
}

impl Gear {
    pub fn init(mut commands: Commands) {
        commands.init_resource::<Option<SetModeState>>();
    }

    pub fn spawn(
        mut commands: Commands,
        assets: Res<GearAssets>,
        gltf_assets: Res<Assets<Gltf>>,
        gltf_mesh_assets: Res<Assets<GltfMesh>>,
    ) {
        let gltf = gltf_assets.get(&assets.gear).unwrap();
        let gltf_mesh = gltf_mesh_assets.get(&gltf.meshes[0]).unwrap();
        commands
            .spawn_bundle(PbrBundle {
                mesh: gltf_mesh.primitives[0].mesh.clone(),
                material: gltf_mesh.primitives[0].material.clone().unwrap(),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
                ..Default::default()
            })
            .insert_bundle(PickableBundle::default())
            .insert(Gear { mode: Job::Idle });
    }


    //TODO: Maybe upgrade to allow dragging into the direction from the rotated grabbed point.
    pub fn set_mode(
        camera_query: Query<&PickingCamera>,
        mut query: Query<(&Gear, &mut Transform)>,
        mouse: Res<Input<MouseButton>>,
        windows: Res<Windows>,
        mut state: ResMut<Option<SetModeState>>,
    ) {
        if mouse.just_pressed(MouseButton::Left) {
            if let Some((entity, intersection)) = camera_query.single().intersect_top() {
                let (gear, transform) = query.get(entity).unwrap();
                let local_coords = transform.with_rotation(Quat::IDENTITY).compute_matrix().inverse().transform_point3(intersection.position());
                info!("{:?}", local_coords);
                let perpendicular = Vec2::new(-local_coords.y, local_coords.x).normalize();
                info!("{:?}", perpendicular);
                let window = windows.get_primary().unwrap();
                let mouse_pos = window.cursor_position().unwrap();
                *state = Some(SetModeState { drag_direction: perpendicular, drag_start: mouse_pos, target: entity, original_rot: transform.rotation });
            }
        } else if mouse.pressed(MouseButton::Left) {
            if let Some(state) = &*state {
                let (gear, mut transform) = query.get_mut(state.target).unwrap();
                let window = windows.get_primary().unwrap();
                if let Some(position) = window.cursor_position() {
                    let drag = position - state.drag_start;
                    let similarity = state.drag_direction.dot(drag.normalize_or_zero());
                    info!("{:?}", similarity);
                    transform.rotation = state.original_rot * Quat::from_rotation_z((similarity*drag.length())/window.width()*5.);
                }

            }
        } else if mouse.just_released(MouseButton::Left) {
            *state = None;
        }
    }
}