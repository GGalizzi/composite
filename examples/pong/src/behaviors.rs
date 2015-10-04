use composite::event::EventManager;
use composite::behavior::Behavior;
use composite::Entity;

use ::world::World;
use ::events::*;

const PADDLE_VELOCITY: f64 = 5.0;

struct InputBehavior;
impl Behavior<World, Event> for InputBehavior {
    fn process(&self, events: Vec<Event>, ent: Entity, world: &mut World, ev_manager: &mut EventManager<Event>) {
        for e in events.into_iter() {
            match e {
                Event::KeyPress(e) => {
                    use piston::input::Key::*;
                    match e.key {
                        Up => {
                            ev_manager.push_for(ent, Event::ChangeVelocity(ChangeVelocity{
                                dx:0.0,dy:-PADDLE_VELOCITY
                            }));
                        },
                        Down => {
                            ev_manager.push_for(ent, Event::ChangeVelocity(ChangeVelocity{
                                dx:0.0,dy:PADDLE_VELOCITY
                            }));
                        },
                        _ => {},
                    }
                },
                Event::KeyRelease(_) => {println!("keyrelease");},
                _ => {},
            }
        }
    }
}

pub struct MoveBehavior;
impl Behavior<World, Event> for MoveBehavior {
    fn process(&self, events: Vec<Event>, ent: Entity, world: &mut World, ev_manager: &mut EventManager<Event>) {
        let ref mut data = world.manager.data[ent];
        for e in events.into_iter() {
            match e {
                Event::ChangeVelocity(ChangeVelocity{dx,dy}) => {
                    data.velocity.dx = dx;
                    data.velocity.dy = dy;
                },
                _ => {},
            }
        }

        data.position.x += data.velocity.dx;
        data.position.y += data.velocity.dy;
    }
}

pub struct BallCollision;
impl Behavior<World, Event> for BallCollision {
    fn process(&self, events: Vec<Event>, ent: Entity, world: &mut World, ev_manager: &mut EventManager<Event>) {
        let ref data = world.manager.data[ent];

        for c in world.manager.data.members_of("collidable") {
            if c == ent { continue; }
            let ref c_data = world.manager.data[c];

            if (data.position.x < c_data.position.x + c_data.dimensions.w &&
                data.position.x + data.dimensions.w > c_data.position.x &&
                data.position.y < c_data.position.y + c_data.dimensions.h &&
                data.dimensions.h + data.position.y > c_data.position.y) {
                println!("collission");
                let ref vel = data.velocity;

                let dx = -vel.dx;
                let dy = -vel.dy;
                ev_manager.push_for(ent, Event::ChangeVelocity(ChangeVelocity{dx:dx,dy:dy}));
            }
        }
    }
}

pub struct AiBehavior;
impl Behavior<World, Event> for AiBehavior {
    fn process(&self, events: Vec<Event>, ent: Entity, world: &mut World, ev_manager: &mut EventManager<Event>) {

        let ball = world.ball;

        let dy = match ball {
            Some(ball) => {
                let ref ball = world.manager.data[ball];
                if ball.velocity.dy > 0.0 {
                    PADDLE_VELOCITY
                } else {
                    -PADDLE_VELOCITY
                }
            }
            None => {
                //Go to center waiting for respawn
                0.0 // TODO
            }
        };

        ev_manager.push_for(ent, Event::ChangeVelocity(ChangeVelocity{dx:0.0, dy:dy}));
    }
}

behaviors!(World:
           [InputBehavior: family: controlled, events: input],
           [AiBehavior: family: ai_controlled, events:],
           [MoveBehavior: family: movable, events: velocity],
           [BallCollision: family: ball, events:]);
