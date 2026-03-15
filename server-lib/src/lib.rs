mod game;

use godot::prelude::*;

struct ServerLibExtension;

#[gdextension]
unsafe impl ExtensionLibrary for ServerLibExtension {}
