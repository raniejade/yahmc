extern crate ecs;
#[macro_use]
extern crate ecs_derive;

use ecs::view::{ReadView, WriteView};
use ecs::system::SystemData;
use ecs::resource::Resources;
use ecs::component::Component;
use ecs::component::storage::VecStorage;

struct SomeResource(i32);
struct AnotherResource(i32);

#[test]
fn fetch_system_data() {
    let mut resources = Resources::new();
    resources.add(SomeResource(0));
    let resource = <ReadView<SomeResource>>::fetch(&resources);
    assert_eq!(resource.0, 0);
}

#[test]
fn fetch_mut_system_data() {
    let mut resources = Resources::new();
    resources.add(SomeResource(0));
    {
        let mut resource = <WriteView<SomeResource>>::fetch(&resources);
        resource.0 = 100;
    }
    let updated_resource = <ReadView<SomeResource>>::fetch(&resources);
    assert_eq!(updated_resource.0, 100);
}

#[test]
fn fetch_system_data_tuple() {
    let mut resources = Resources::new();
    resources.add(SomeResource(0));
    resources.add(AnotherResource(1));
    let (some, mut another) = <(ReadView<SomeResource>, WriteView<AnotherResource>)>::fetch(&resources);
    assert_eq!(some.0, 0);
    assert_eq!(another.0, 1);
}