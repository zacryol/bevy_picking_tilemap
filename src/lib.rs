use bevy::{
    app::{Plugin, PreUpdate},
    ecs::{
        entity::Entity,
        event::EventWriter,
        query::With,
        system::{Query, Single},
    },
    math::{Vec2, Vec4},
    picking::{
        backend::{HitData, PointerHits},
        pointer::{PointerId, PointerLocation},
        PickSet, Pickable,
    },
    prelude::{App, IntoScheduleConfigs, Vec4Swizzles},
    render::camera::Projection,
    render::{camera::Camera, view::ViewVisibility},
    transform::components::GlobalTransform,
    window::PrimaryWindow,
};
use bevy_ecs_tilemap::{
    anchor::TilemapAnchor,
    map::{TilemapGridSize, TilemapSize, TilemapTileSize, TilemapType},
    tiles::{TilePos, TileStorage, TileVisible},
};

pub use bevy_ecs_tilemap;

/// picking backend for `bevy_ecs_tilemap`
///
/// The plugins for picking and tilemaps must be added separately
pub struct TilemapBackend;

impl Plugin for TilemapBackend {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, tile_picking.in_set(PickSet::Backend));
    }
}

#[expect(clippy::type_complexity)]
fn tile_picking(
    pointers: Query<(&PointerId, &PointerLocation)>,
    cameras: Query<(Entity, &Camera, &GlobalTransform, &Projection)>,
    primary_window: Option<Single<Entity, With<PrimaryWindow>>>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapTileSize,
        &TilemapType,
        &TilemapAnchor,
        &TileStorage,
        &GlobalTransform,
        &ViewVisibility,
    )>,
    tile_q: Query<(&TileVisible, Option<&Pickable>)>,
    mut output: EventWriter<PointerHits>,
) {
    for (p_id, p_loc) in pointers
        .iter()
        .filter_map(|(p_id, p_loc)| p_loc.location().map(|l| (p_id, l)))
    {
        let mut blocked = false;
        let Some((cam_entity, camera, cam_transform, cam_ortho)) = cameras
            .iter()
            .filter(|(_, camera, _, _)| camera.is_active)
            .find(|(_, camera, _, _)| {
                camera
                    .target
                    .normalize(primary_window.as_deref().copied()
                    )
                    .is_some_and(|p| p == p_loc.target)
            })
        else {
            continue;
        };

        let Ok(cursor_pos_world) = camera.viewport_to_world_2d(cam_transform, p_loc.position)
        else {
            continue;
        };

        let picks = tilemap_q
            .iter()
            .filter(|(.., vis)| vis.get())
            .filter_map(|(t_s, tgs, tts, tty, t_anchor, t_store, gt, _)| {
                if blocked {
                    return None;
                }
                let in_map_pos: Vec2 = {
                    let pos = Vec4::from((cursor_pos_world, 0., 1.));
                    let in_map_pos = gt.compute_matrix().inverse() * pos;
                    in_map_pos.xy()
                };
                let picked: Entity =
                    TilePos::from_world_pos(&in_map_pos, t_s, tgs, tts, tty, t_anchor)
                        .and_then(|tile_pos| t_store.get(&tile_pos))?;
                let (vis, pck) = tile_q.get(picked).ok()?;
                if !vis.0 {
                    return None;
                }
                blocked = pck.is_some() && matches!(pck, Some(&Pickable::IGNORE));

                let depth = -match cam_ortho {
                    Projection::Orthographic(orth) => orth.near,
                    Projection::Perspective(per) => per.near, // TODO: is this correct?
                    Projection::Custom(_) => todo!("idk"),
                } - gt.translation().z;
                Some((picked, HitData::new(cam_entity, depth, None, None)))
            })
            .collect();

        let order = camera.order as f32;
        output.write(PointerHits::new(*p_id, picks, order));
    }
}
