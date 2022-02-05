pub mod container;
pub mod deferred;

#[macro_use]
pub mod inject;

pub use waiter_codegen::*;

pub use container::*;
pub use deferred::*;
pub use inject::*;
use std::any::Any;

pub type Wrc<T> = std::sync::Arc<T>;
pub type RcAny = Wrc<dyn Any + Send + Sync>;
