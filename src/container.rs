use crate::{RcAny, Wrc};
use std::any::TypeId;
use std::collections::HashMap;

pub trait Component {
    fn __waiter_create<P>(container: &mut Container) -> Self;
    fn __waiter_inject_deferred<P>(container: &mut Container, component: &Self);
}

pub trait Provider<T: ?Sized> {
    type Impl;
    fn get(&mut self) -> Wrc<Self::Impl>;
    fn create(&mut self) -> Self::Impl;

    fn get_ref(&mut self) -> &Self::Impl {
        // Value under RC is still stored in container, so it can be safely returned as a reference
        // that has the same life as container reference
        unsafe { Wrc::as_ptr(&Self::get(self)).as_ref().unwrap() }
    }
    fn create_boxed(&mut self) -> Box<Self::Impl> {
        Box::new(Self::create(self))
    }
}

pub struct Container {
    pub components: HashMap<TypeId, RcAny>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            components: HashMap::new(),
        }
    }
}
