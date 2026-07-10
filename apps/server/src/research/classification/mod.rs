mod controller;
mod entity;

pub use controller::*;
pub use entity::*;

mod dtos {
    mod fields;
    mod keywords;
    mod subfields;
    mod topics;

    pub use fields::*;
    pub use subfields::*;
    pub use topics::*;
}

pub use dtos::*;

mod services {
    mod domains;
    mod fields;
    mod keywords;
    mod subfields;
    mod topics;

    pub use domains::*;
    pub use fields::*;
    pub use keywords::*;
    pub use subfields::*;
    pub use topics::*;
}

pub use services::*;

mod repositories {
    mod domains;
    mod fields;
    mod keywords;
    mod subfields;
    mod topics;

    pub use domains::*;
    pub use fields::*;
    pub use keywords::*;
    pub use subfields::*;
    pub use topics::*;
}

pub use repositories::*;
