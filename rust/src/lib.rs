use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

mod wfc_map;
mod wfc_relation;
mod wfc_tile_dictionary;
