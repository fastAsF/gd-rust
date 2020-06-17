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
