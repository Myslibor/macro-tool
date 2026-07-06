use std::usize;

use rdev::Key;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Brick {
    pub button: String,
    pub wait: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Macro {
    pub bricks: Vec<Brick>,
    pub key_bind: String,
    pub has_loop: bool,
}
