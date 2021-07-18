mod root;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<root::Root>();
}

godot_init!(init);