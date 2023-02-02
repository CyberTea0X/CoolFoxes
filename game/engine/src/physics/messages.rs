#[derive(Debug, Clone, Copy)]
pub struct PhysicsMessage {
    pub force: (f64, f64)
}

impl PhysicsMessage {
    pub fn apply_force<A: Into<f64>, B: Into<f64>>(x: A, y: B) -> PhysicsMessage {
        return PhysicsMessage{force: (x.into(), y.into())}
    }
}
