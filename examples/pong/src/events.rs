use piston::input::Key;

#[derive(Debug, Clone)]
pub struct KeyPress {
    pub key: Key,
}

impl KeyPress {
    pub fn new(key: Key) -> KeyPress {
        KeyPress{key:key}
    }
}

#[derive(Debug, Clone)]
pub struct KeyRelease;

#[derive(Debug, Clone)]
pub struct ChangeVelocity {
    pub dx: f64,
    pub dy: f64,
}

events!([input, KeyPress],
        [input, KeyRelease],
        [velocity, ChangeVelocity]);
