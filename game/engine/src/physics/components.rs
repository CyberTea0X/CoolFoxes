#[derive(Debug)]
pub enum PhysicsComponent {
    GravityComponent(GravityComponent)
}
pub struct PhysicsMessage;

#[derive(Debug)]
pub struct GravityComponent;