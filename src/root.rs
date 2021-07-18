use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register_signals)]
#[user_data(user_data::MutexData<Root>)]
pub struct Root {}

#[gdnative::methods]
impl Root {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "init",
            args: &[],
        });
    }

    fn new(_owner: &Spatial) -> Self {
        godot_print!("created root");
        Root {
        }
    }

    #[export]
    fn _ready(&self, _owner: &Spatial) {
        godot_print!("root ready");
    }

}