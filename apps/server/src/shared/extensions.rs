use crate::auth::SessionClaims;
use sword::web::Request;

pub trait RequestExt {
    // fn user(&self) -> Option<&User>;
    fn claims(&self) -> Option<&SessionClaims>;
}

impl RequestExt for Request {
    // fn user(&self) -> Option<&User> {
    //     self.extensions.get::<User>()
    // }

    fn claims(&self) -> Option<&SessionClaims> {
        self.extensions.get::<SessionClaims>()
    }
}
