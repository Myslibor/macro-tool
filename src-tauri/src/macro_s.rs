use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Brick {
    button: String,
    wait: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Macro {
    bricks: Vec<Brick>,
    key_bind: String,
    has_loop: bool,
}
