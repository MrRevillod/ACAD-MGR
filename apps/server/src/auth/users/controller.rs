use crate::auth::*;
use crate::shared::RequestExt;

use std::sync::Arc;
use sword::prelude::*;
use sword::web::*;

#[controller(kind = Controller::Web, path = "/users")]
#[interceptor(SessionCheck)]
pub struct UsersController {
    users: Arc<UsersService>,
}

impl UsersController {
    #[get("/")]
    pub async fn get_users(&self, req: Request) -> WebResult<Vec<UserView>> {
        let query = req.query_validator::<GetUsersQuery>()?;
        let users = self.users.find(query.unwrap_or_default()).await?;

        Ok(users)
    }

    #[get("/{id}")]
    pub async fn get_user(&self, req: Request) -> WebResult<UserView> {
        let id = req.param::<UserId>("id")?;
        let user = self.users.find_by_id(&id).await?;

        Ok(user)
    }

    #[get("/me")]
    pub async fn get_me(&self, req: Request) -> WebResult {
        let claims = req.claims().ok_or_else(JsonResponse::Unauthorized)?;
        let user = self.users.find_by_id(&claims.user_id).await?;

        Ok(JsonResponse::Ok().data(user))
    }

    #[post("/")]
    pub async fn create_user(&self, req: Request) -> WebResult<UserView> {
        let dto = req.body_validator::<CreateUserDto>()?;
        let user = self.users.create(dto).await?;

        Ok(user)
    }
}
