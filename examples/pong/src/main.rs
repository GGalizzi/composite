extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

#[macro_use]
extern crate composite;

use piston::window::WindowSettings;
use piston::input::Event as PEvent;
use piston::input::{Input, Button};
use piston::event_loop::{EventLoop,Events};
use glutin_window::GlutinWindow;
use graphics::{Ellipse,Rectangle};
use opengl_graphics::{GlGraphics, OpenGL};

use composite::event::EventManager;
use composite::EntityDataHolder;

mod world;
mod behaviors;
mod events;
mod components;

use components::*;
use events::*;
use behaviors::behavior_list;
use world::World;

const OPEN_GL: OpenGL = OpenGL::V2_1;
const PADDLE_DIMENSIONS: [f64; 2] = [25.0,97.0];

families!([controlled: key_input -],
          [drawable: dimensions, position -],
          [movable: velocity, position -],
          [ball: round -],
          [collidable: position -]);

prototypes!(World[manager]
            [paddle:
             Dimensions::new(PADDLE_DIMENSIONS[0], PADDLE_DIMENSIONS[1]),
             Position::new(),
             Velocity::new()],
            [ball:
             Dimensions::new(15.0,15.0),
             Round,
             Position::new(),
             Velocity::new()]);

impl<'a> Build<'a> {
    fn at(self, x: f64, y: f64) -> Self {
        {let ref mut data = self.manager.manager.data[self.entity];
        data.position.x = x;
        data.position.y = y;}
        self
    }

    fn set_as_player(self, processor: &mut BehaviorManager<World,Event>) -> Self {
        /*self.manager.manager.build_ent(self.entity, processor)
        .add_component(components::KeyInput).finalize();*/
        self.manager.manager.add_component_to(self.entity, components::KeyInput, processor);
        self
    }
}

fn main() {
    let window: GlutinWindow = WindowSettings::new("Pong", [800,600])
        .exit_on_esc(true)
        .opengl(OPEN_GL)
        .build().unwrap();

    let mut gl = GlGraphics::new(OPEN_GL);

    let mut world = World::new();
    let mut ev_manager: EventManager<Event> = EventManager::new();
    let mut processor: BehaviorManager<World, Event> = BehaviorManager::new(behavior_list());

    let player = Build::paddle(&mut processor, &mut world).at(25.0,80.0).set_as_player(&mut processor).get_id();
    Build::paddle(&mut processor, &mut world).at(725.0, 80.0);
    let ball = Build::ball(&mut processor, &mut world).at(400.0, 80.0).get_id();

    ev_manager.push_for(ball, Event::ChangeVelocity(ChangeVelocity{dx:-8.0, dy:4.0}));
    
    for e in window.events().ups(60).max_fps(60) {
        match e {
            PEvent::Input(Input::Press(Button::Keyboard(key))) => {
                ev_manager.push_for(player, Event::KeyPress(KeyPress::new(key)));
            },
            PEvent::Input(Input::Release(Button::Keyboard(key))) => {
            },
            PEvent::Render(args) => {
                graphics::clear([0.0,0.0,0.0,1.0],&mut gl);

                gl.draw(args.viewport(), |c, g| {
                    for d in world.manager.data.members_of("drawable").into_iter() {
                        let ref d = world.manager.data[d];

                        if d.round.has_it() {
                            Ellipse::new([0.0,1.0,0.0,1.0])
                                .draw([d.position.x, d.position.y,
                                       d.dimensions.w, d.dimensions.h],
                                      &c.draw_state,
                                      c.transform, g);
                        } else {
                            Rectangle::new([1.0,0.0,0.0,1.0])
                                .draw([d.position.x, d.position.y,
                                       d.dimensions.w, d.dimensions.h],
                                      &c.draw_state,
                                      c.transform, g);
                        }
                    }
                });
            }
            PEvent::Update(args) => {
                process_behaviors(&processor, &mut world, &mut ev_manager);
            }
            _ => {}
        }
    }
}

fn process_behaviors(processor: &BehaviorManager<World, Event>,
                     world: &mut World,
                     event_manager: &mut EventManager<Event>) {
    for ent in processor.processable.iter() {
        for beh_idx in processor.valid_behaviors_for(world.manager.data[*ent].families()) {
            let ref behavior = processor.behaviors[beh_idx];
            let relevant_events = event_manager.for_behavior_of(behavior.events(), *ent, false);
            behavior.process(relevant_events, *ent, world, event_manager);
        }
    }
}
