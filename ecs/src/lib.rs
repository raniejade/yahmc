#[macro_use]
extern crate ecs_derive;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate mopa;
extern crate fxhash;
extern crate bit_set;

mod resource;
mod system;
mod component;
mod entity;
mod view;