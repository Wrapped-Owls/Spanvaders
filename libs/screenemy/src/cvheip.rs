use godot::engine::Node2D;
use godot::engine::Node2DVirtual;
use godot::prelude::*;
use std::str::FromStr;

#[derive(GodotClass)]
#[class(base = Node2D)]
struct CVHeip {
    #[base]
    node2d: Base<Node2D>,
}

#[godot_api]
impl Node2DVirtual for CVHeip {
    fn init(node2d: Base<Self::Base>) -> Self {
        Self { node2d }
    }

    fn process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut velocity = Vector2::new(0.0, 0.0);

        if Input::is_action_pressed(&input, StringName::from_str("left").unwrap()) {
            velocity.x -= 1.0;
        }
        if Input::is_action_pressed(&input, StringName::from_str("right").unwrap()) {
            velocity.x += 1.0;
        }
        if Input::is_action_pressed(&input, StringName::from_str("up").unwrap()) {
            velocity.y -= 1.0;
        }
        if Input::is_action_pressed(&input, StringName::from_str("down").unwrap()) {
            velocity.y += 1.0;
        }

        let pos = self.node2d.get_position();
        self.node2d.set_position(pos + velocity);
    }
}
