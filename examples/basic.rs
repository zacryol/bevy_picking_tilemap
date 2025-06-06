//! Example of tiles receiving pick events
//! Click on a tile to change its texture.

use bevy::prelude::*;
use bevy_picking_tilemap::{bevy_ecs_tilemap::prelude::*, TilemapBackend};
use rand::Rng;

const TILE_COUNT: u32 = 1078;

/// mostly the same as the `basic` example from `bevy_ecs_tilemap`
fn tilemap_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Asset by Kenney
    let texture_handle: Handle<Image> = asset_server.load("colored_packed.png");
    let map_size = TilemapSize { x: 32, y: 32 };

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(0),
                    ..default()
                },))
                // The important part; each Tile has a picking handler to change the tile
                // texture.
                .observe(
                    |trigger: Trigger<Pointer<Click>>,
                     mut tile_query: Query<&mut TileTextureIndex>| {
                        let entity = trigger.target();
                        if let Ok(mut texture_index) = tile_query.get_mut(entity) {
                            let mut rng = rand::rng();
                            texture_index.0 = rng.random_range(0..TILE_COUNT);
                        }
                    },
                )
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16., y: 16. };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        anchor: TilemapAnchor::Center,
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            TilemapPlugin,
            // The additional backend to check events against the tiles
            TilemapBackend,
        ))
        .add_systems(
            Startup,
            (tilemap_startup, |mut commands: Commands| {
                commands.spawn((
                    Camera2d,
                    Projection::Orthographic(OrthographicProjection {
                        scale: 0.5,
                        ..OrthographicProjection::default_2d()
                    }),
                ));
            }),
        )
        .run();
}
