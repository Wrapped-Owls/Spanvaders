use godot::engine::Node2D;
use godot::engine::Node2DVirtual;
use godot::prelude::*;

use corevaders::controller::command::IOCommand;

#[derive(GodotClass)]
#[class(base = Node2D)]
struct CVHeip {
    #[base]
    node2d: Base<Node2D>,
}

const MOVE_STEP: f32 = 2.0;

#[godot_api]
impl Node2DVirtual for CVHeip {
    fn init(node2d: Base<Self::Base>) -> Self {
        Self { node2d }
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();
        let mut velocity = Vector2::new(0.0, 0.0);

        if Input::is_action_pressed(&input, IOCommand::Left.as_godot_str()) {
            velocity.x -= MOVE_STEP;
        }
        if Input::is_action_pressed(&input, IOCommand::Right.as_godot_str()) {
            velocity.x += MOVE_STEP;
        }
        if Input::is_action_pressed(&input, IOCommand::Up.as_godot_str()) {
            velocity.y -= MOVE_STEP;
        }
        if Input::is_action_pressed(&input, IOCommand::Down.as_godot_str()) {
            velocity.y += MOVE_STEP;
        }

        let pos = self.node2d.get_position();
        self.node2d.set_position(pos + velocity);
    }
}
