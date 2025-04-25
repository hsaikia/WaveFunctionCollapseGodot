use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

mod wfc_map;
mod wfc_probability_map;
mod wfc_tile_dictionary;
