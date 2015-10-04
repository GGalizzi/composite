#[derive(Clone, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new() -> Position {
        Position{x:0.0,y:0.0}
    }
}

#[derive(Clone, Debug)]
pub struct Dimensions {
    pub w: f64,
    pub h: f64,
}

impl Dimensions {
    pub fn new(w: f64, h: f64) -> Dimensions {
        Dimensions { w:w, h:h }
    }
}

#[derive(Clone, Debug)]
pub struct KeyInput;

#[derive(Clone, Debug)]
pub struct Velocity {
    pub dx: f64,
    pub dy: f64,
}

impl Velocity {
    pub fn new() -> Velocity {
        Velocity{dx:0.0, dy:0.0}
    }
}

#[derive(Clone,Debug)]
pub struct Round;

#[derive(Clone, Debug)]
pub struct Ai;

#[macro_use]
components!([position, Position],
            [dimensions, Dimensions],
            [round, Round],
            [key_input, KeyInput],
            [ai,Ai],
            [velocity, Velocity]);
