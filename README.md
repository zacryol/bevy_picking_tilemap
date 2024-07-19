# `bevy_picking_tilemap`

Provides a [`bevy_mod_picking`](https://github.com/aevyrie/bevy_mod_picking) backend to add
integration with [`bevy_ecs_tilemap`](https://github.com/StarArawn/bevy_ecs_tilemap), enabling individual
Tile entities to receive the picking events.

## How to use

In addition to the plugins for the other two crates, simply add the `TilemapBackend` to the App. Then, any entity with the `TileBundle`
should be able to receive picking events.
