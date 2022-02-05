extern crate serde;
extern crate waiter_di;

use std::sync::Arc;

use waiter_di::*;

trait Interface: Send {
    fn demo(&self);
}

#[component]
struct InterfaceImpl {}

#[provides]
impl Interface for InterfaceImpl {
    fn demo(&self) {
        println!("Dependency");
    }
}

#[component]
struct SomeComp {
    interface: Arc<dyn Interface>,
}

fn main() {
    let mut container = Container::new();

    let component = Provider::<SomeComp>::get(&mut container);

    component.interface.demo();
}
