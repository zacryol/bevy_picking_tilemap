# `bevy_picking_tilemap`

Provides a picking backend to add integration with [`bevy_ecs_tilemap`](https://github.com/StarArawn/bevy_ecs_tilemap),
enabling individual Tile entities to receive the picking triggers.

## How to use

In addition to the plugins for picking and `bevy_ecs_tilemap`, simply add the `TilemapBackend` to the App. Then, any entity with the `TileBundle`
should be able to receive picking events.

- See the `basic` example for more detail.
