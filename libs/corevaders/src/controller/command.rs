use std::str::FromStr;

use godot::prelude::StringName;

pub enum IOCommand {
    Left,
    Right,
    Up,
    Down,
}

impl IOCommand {
    pub fn as_str(&self) -> &str {
        match self {
            IOCommand::Left => "left",
            IOCommand::Right => "right",
            IOCommand::Up => "up",
            IOCommand::Down => "down",
        }
    }

    pub fn as_godot_str(&self) -> StringName {
        return StringName::from_str(self.as_str()).unwrap_or_default();
    }
}
