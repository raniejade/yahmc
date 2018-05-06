extern crate ecs;
#[macro_use]
extern crate ecs_derive;

use ecs::component::Component;
use ecs::component::storage::VecStorage;
use std::any::TypeId;

#[derive(Component)]
#[Storage(VecStorage)]
struct MyComponent(i32);

#[test]
fn storage_match() {
    assert_eq!(
        TypeId::of::<VecStorage<MyComponent>>(),
        TypeId::of::<<MyComponent as Component>::Storage>()
    );
}
