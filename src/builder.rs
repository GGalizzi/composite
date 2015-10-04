//! Implements `Build` an interface to define prototypes
//!
//! With the `prototypes!` macro you can predefine a type of reusable entity.
//! Like a tree, a rock, or an orc.
//! 
//! # Examples
//!
//! ```
//! prototypes!([tree: Glyph{ch:'t'}, Position{ .. Default::default()}],
//!             [orc: Glyph{ch:'o'}, Position{ .. Default::default()},
//!             Stats{hp: 25, atk: 10, .. Default::default()}]);
//!
//! let ent = Build::tree(&mut manager).get_id();
//! ```
//!
//! This would generate a `Build` struct with one static function per defined prototype, this function
//! returns the `Build` itself, to get the `Entity` from it you can call `get_id`.
//!
//! The builder function also takes a `&mut EntityManager<A,B>`, which is the manager the entity will be added to.
//!
//! You can expand on the generated `Build` type to make some helper functions, for example:
//! 
//! ```
//! impl<'a> Build<'a> {
//!     fn at(self, x:i32, y:i32) -> Self {
//!         self.data.position.x = x;
//!         self.data.position.y = y;
//!         self
//!     }
//! }
//! //...
//! Build::orc(&mut manager).at(10,15);
//! ```

/// See the [builder module documentation](/ecs/builder/) for information on this macro.
#[macro_export]
macro_rules! prototypes {
    ($t:ty[$manager:ident] $([$proto:ident: $($comp:expr),+]),+) => {

        use $crate::Component;
        struct Build<'a> {
            entity: $crate::Entity,
            manager: &'a mut $t,
        }



        impl<'a> Build<'a> {
            pub fn new(ent: $crate::Entity, manager: &'a mut $t) -> Build<'a> {
                Build{
                    entity: ent,
                    manager: manager,
                }
            }

            #[allow(dead_code)]
            pub fn get_id(self) -> $crate::Entity {
                self.entity
            }
            $(
                fn $proto(processor: &mut BehaviorManager<$t, Event>, manager: &'a mut $t) -> Build<'a> {
                    let ent = manager.$manager.new_entity();
                    {
                        manager.$manager.build_ent(ent, processor)
                        $(
                            .add_component($comp)
                            //manager.add_component($comp).to(ent, processor);
                            )+

                            .finalize();
                    }
                    Build::new(ent, manager)
                }
             )+

         
        }
    }
}
