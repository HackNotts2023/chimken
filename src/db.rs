pub mod comment;
pub use comment::*;

#[macro_use]
pub mod id;
pub use id::*;

pub mod restaurant;
pub use restaurant::*;

pub mod user;
pub use user::*;

pub mod votes;
pub use votes::*;

const DATABASE: &str = "chimken";