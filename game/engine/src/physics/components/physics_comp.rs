use cgmath::{Point2, Vector2};
use measurements::{Mass};
use queues::Queue;
use crate::component::Component;
use crate::component::messages::ComponentMessage;
use crate::graphics::Sprite;
use crate::messaging::traits::Messaging;
use crate::misc_traits::update::Updatable;
use crate::rect::Rectangular;

#[derive(Debug)]
pub struct PhysicsComponent {
    time_elapsed: u32,
    mass: Mass,
    gravity: f64,
    velocity: Vector2<f64>,
    acceleration: Vector2<f64>,
    input_messages: Queue<ComponentMessage>,
    output_messages: Queue<ComponentMessage>
}

impl PhysicsComponent {
    pub fn new(mass: Mass, gravity: f64) -> Component {
        return Component::PhysicsComponent(PhysicsComponent
        {
            time_elapsed: 0,
            mass,
            gravity,
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            input_messages: Queue::new(),
            output_messages: Queue::new(),
        })
    }
    /// Ускоряем объект
    fn accelerate(&mut self, acc: Vector2<f64>) {
        self.velocity += acc;
    }
    /// Ускоряем объект в зависимости от силы
    fn apply_force(&mut self, f: Vector2<f64>) {
        let a = f / self.mass.as_kilograms();
        self.accelerate(a);
    }
    /// Применяем гравитацию к объекту
    fn apply_gravity(&mut self, dt: f64) {
        let gravity_force = self.gravity * self.mass.as_kilograms() * dt;
        self.apply_force(Vector2::new(0.0, gravity_force))
    }
    /// Не даёт объектам выпасть за экран
    fn apply_constraint(host: &mut Sprite) {
        if host.y() > 768.0 {
            host.move_ip(None, Some(768));
        }
    }
}

impl Updatable for PhysicsComponent {
    type Host = Sprite;
    fn get_time_elapsed(&self) -> u32 {
        self.time_elapsed
    }
    fn get_time_elapsed_mut(&mut self) -> &mut u32 {
        &mut self.time_elapsed
    }
    fn get_delay(&self) -> u32 {
        1
    }
    fn updated_internal(mut self, dt: u32, _host: &mut Self::Host) -> Self where Self: Sized {
        let dt = dt as f64 / 1000.0;
        let mut position_current = _host.position();
        let a = Vector2::new(0.0, self.gravity);
        let mut v = self.velocity;
        let distance = v * dt + 0.5 * a * dt * dt;
        v += 0.5 * a * dt;
        v += 0.5 * a * dt;
        self.velocity = v;
        _host.move_by(distance.x, distance.y);
        PhysicsComponent::apply_constraint(_host);
        self
    }
}

impl Messaging for PhysicsComponent {
    type Message = ComponentMessage;
    fn input_messages(&mut self) -> &mut Queue<Self::Message> {
        &mut self.input_messages
    }
    fn output_messages(&mut self) -> &mut Queue<Self::Message> {
        &mut self.output_messages
    }
    fn handle_message(&mut self, message: Self::Message) {
        match message {
            ComponentMessage::PhysicsMessage(p) => {
            },
            (_) => {}
        }
    }
}
