use rdev::Key;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Brick {
    button: String,
    wait: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Macro {
    pub bricks: Vec<Brick>,
    pub key_bind: String,
    pub has_loop: bool,
}
