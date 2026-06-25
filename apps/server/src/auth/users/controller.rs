use crate::auth::*;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = Controller::Web, path = "/users")]
pub struct UsersController {
    users: Arc<UsersService>,
}

impl UsersController {
    #[get("/")]
    #[interceptor(SessionCheck)]
    pub async fn get_users(&self, req: Request) -> WebResult<Vec<UserView>> {
        let query = req.query_validator::<GetUsersQuery>()?;
        let users = self.users.find(query.unwrap_or_default()).await?;

        Ok(users)
    }

    #[get("/{id}")]
    #[interceptor(SessionCheck)]
    pub async fn get_user(&self, req: Request) -> WebResult<UserView> {
        let id = req.param::<UserId>("id")?;
        let user = self.users.find_by_id(&id).await?;

        Ok(user)
    }

    #[post("/")]
    #[interceptor(SessionCheck)]
    pub async fn create_user(&self, req: Request) -> WebResult<UserView> {
        let dto = req.body_validator::<CreateUserDto>()?;
        let user = self.users.create(dto).await?;

        Ok(user)
    }
}
