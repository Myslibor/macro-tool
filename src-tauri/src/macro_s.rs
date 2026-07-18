use std::{usize, vec};

use rdev::Key;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Brick {
    pub button: String,
    pub wait: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Macro {
    pub bricks: Vec<Brick>,
    pub key_bind: Vec<String>,
    pub name: String,
    pub has_loop: bool,
}

impl Macro {
    pub fn new() -> Macro {
        Macro {
            bricks: Vec::new(),
            key_bind: Vec::new(),
            name: "Default name".into(),
            has_loop: false,
        }
    }
}
