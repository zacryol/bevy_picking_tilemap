# `bevy_picking_tilemap`

Provides a [`bevy_mod_picking`](https://github.com/aevyrie/bevy_mod_picking) backend to add
integration with [`bevy_ecs_tilemap`](https://github.com/StarArawn/bevy_ecs_tilemap), enabling individual
Tile entities to receive the picking events.

 - **Note:** `bevy_ecs_tilemap`'s `bevy 0.13` support has not been uploaded to crates.io. The crate must be added to dependencies
   via the GitHub repo.
   
   - It is also re-exported by this crate.


## How to use

In addition to the plugins for the other two crates, simply add the `TilemapBackend` to the App. Then, any entity with the `TileBundle`
should be able to receive picking events.
