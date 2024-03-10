//! Example of tiles receiving pick events
//! Click on a tile to change its texture.

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_picking_tilemap::TilemapBackend;
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
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(0),
                        ..default()
                    },
                    // The important part; each Tile has a picking handler to change the tile
                    // texture.
                    On::<Pointer<Click>>::target_component_mut::<TileTextureIndex>(|_, t| {
                        let mut rng = rand::thread_rng();
                        t.0 = rng.gen_range(0..TILE_COUNT);
                    }),
                ))
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
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            TilemapPlugin,
            DefaultPickingPlugins,
            // The additional backend to check events against the tiles
            TilemapBackend,
        ))
        .add_systems(
            Startup,
            (tilemap_startup, |mut commands: Commands| {
                let mut cb = Camera2dBundle::default();
                cb.projection.scale = 0.5;
                commands.spawn(cb);
            }),
        )
        .run();
}
