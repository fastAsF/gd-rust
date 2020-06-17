use gdnative::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct InjectClass;

#[methods]
impl InjectClass {
    fn _init(_owner: Node) -> Self {
        InjectClass
    }

    #[export]
    fn _ready(&self, _owner: Node) {
        godot_print!("Injected to Node");
    }
}

fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<InjectClass>();
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
