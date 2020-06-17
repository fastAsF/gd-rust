use gdnative::*;

mod inject_class;
// mod player;

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<inject_class::InjectClass>();
    // handle.add_class::<player::Player>();
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
