use gdnative::*;

#[derive(NativeClass)]
#[inherit(Area2D)]
#[user_data(user_data::MutexData<Player>)]
#[register_with(Self::register_player)]
pub struct Player {
    #[property(default = 400.0)]
    sensitivity_x: f64,
    sensitivity_y: f64,
    invert_y_axis: bool,
    exit_on_escape: bool,
    maximum_y_look: f64,
    accelaration: i64,
    jump_speed: i64,
    velocity: Vector3,
    forward_velocity: i64,
    walk_speed: i64,
    maximum_walk_speed: i64,
    screen_size: Vector2,
}

#[methods]
impl Player {
    fn register_player(builder: &init::ClassBuilder<Self>) {
        builder.add_signal(init::Signal {
            name: "hit",
            args: &[],
        });
    }

    fn _init(_owner: Area2D) -> Self {
        Player {
            walk_speed: 400,
            velocity: Vector3::new(0., 0., 0.),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: Area2D) {
        owner.hide();
    }

    #[export]
    unsafe fn _physics_process(&mut self, owner: Area2D, delta: f32) {
        self.velocity.y -= 0.098;
        let input = Input::godot_singleton();
        if Input::is_action_pressed(&input, GodotString::from_str("ui_up")) {
            self.walk_speed += self.accelaration;
            if self.walk_speed > self.maximum_walk_speed {
                self.walk_speed = self.maximum_walk_speed;
            }
            self.velocity.x =
                -gdnative::Spaital::get_global_transform().basis.z.x * self.walk_speed;
            self.velocity.z =
                -gdnative::Spaital::get_global_transform().basis.z.z * self.walk_speed;
        }
        if Input::is_action_pressed(&input, GodotString::from_str("ui_down")) {
            self.walk_speed += self.accelaration;
            if self.walk_speed > self.maximum_walk_speed {
                self.walk_speed = self.maximum_walk_speed;
            }
            self.velocity.x = gdnative::Spaital::get_global_transform().basis.z.x * self.walk_speed;
            self.velocity.z = gdnative::Spaital::get_global_transform().basis.z.z * self.walk_speed;
        }
        if Input::is_action_pressed(&input, GodotString::from_str("ui_left")) {
            self.walk_speed += self.accelaration;
            if self.walk_speed > self.maximum_walk_speed {
                self.walk_speed = self.maximum_walk_speed;
            }
            self.velocity.x =
                -gdnative::Spaital::get_global_transform().basis.x.x * self.walk_speed;
            self.velocity.z =
                -gdnative::Spaital::get_global_transform().basis.x.z * self.walk_speed;
        }
        if Input::is_action_pressed(&input, GodotString::from_str("ui_right")) {
            self.walk_speed += self.accelaration;
            if self.walk_speed > self.maximum_walk_speed {
                self.walk_speed = self.maximum_walk_speed;
            }
            self.velocity.x = gdnative::Spaital::get_global_transform().basis.x.x * self.walk_speed;
            self.velocity.z = gdnative::Spaital::get_global_transform().basis.x.z * self.walk_speed;
        }

        if !Input::is_action_pressed(&input, GodotString::from_str("ui_right"))
            && !Input::is_action_pressed(&input, GodotString::from_str("left"))
            && !Input::is_action_pressed(&input, GodotString::from_str("ui_up"))
            && !Input::is_action_pressed(&input, GodotString::from_str("ui_down"))
        {
            self.velocity.x = 0.;
            self.velocity.z = 0.;
        }

        self.velocity =
            gdnative::KinematicBody::move_and_slide(&self.velocity, Vector3::new(0, 1, 0));
    }
}
