mod cookies;
mod hasher;

pub use cookies::*;
pub use hasher::*;

use crate::{auth::*, shared::*};

use chrono::{DateTime, Duration, Utc};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use sword::prelude::*;

#[injectable]
pub struct AuthService {
    config: AuthConfig,
    users: Arc<UsersRepository>,
    jwt_service: Arc<JsonWebTokenService>,
    sessions: Arc<SessionRepository>,
    hasher: Arc<Hasher>,
}

impl AuthService {
    pub async fn login(&self, input: &LoginDto) -> AppResult<LoginResponse> {
        let LoginDto { email, password } = input;

        let Some(user) = self.users.find_by_email(email).await? else {
            return Err(AuthError::InvalidCredentials)?;
        };

        if !self.hasher.verify(password, &user.password_hash)? {
            return Err(AuthError::InvalidCredentials)?;
        }

        let session_id = SessionId::new();

        let (access_token, access_token_exp) = self.generate_access_token(&session_id, &user.id)?;
        let (refresh_token, refresh_token_exp) =
            self.generate_refresh_token(&session_id, &user.id)?;

        let now = Utc::now();

        let session = Session {
            id: session_id,
            user_id: user.id,
            refresh_token_hash: Self::hash_token(&refresh_token),
            created_at: now,
            expires_at: access_token_exp,
            refresh_expires_at: refresh_token_exp,
            revoked_at: None,
        };

        self.sessions.save(&session).await?;

        Ok(LoginResponse {
            user: user.into(),
            access_token,
            access_token_exp,
            refresh_token,
            refresh_token_exp,
        })
    }

    pub async fn refresh(&self, token: &String) -> AppResult<RefreshResponse> {
        let claims = self
            .jwt_service
            .decode::<SessionClaims>(token, self.config.jwt_secret.as_ref())?;

        if claims.typ != "refresh" {
            return Err(AuthError::InvalidToken)?;
        }

        let session = self
            .sessions
            .find_active_by_refresh_id(&claims.session_id)
            .await?
            .ok_or(AuthError::TokenNotFound)?;

        let (access_token, access_token_exp) =
            self.generate_access_token(&session.id, &session.user_id)?;

        self.sessions
            .update_expires_at(&session.id, access_token_exp)
            .await?;

        Ok(RefreshResponse {
            access_token,
            access_token_exp,
        })
    }

    pub async fn logout(&self, session_id: &SessionId) -> AppResult<()> {
        if let Some(mut session) = self.sessions.find_active_by_id(session_id).await? {
            session.revoked_at = Some(Utc::now());
            self.sessions.save(&session).await?;
        }

        Ok(())
    }

    fn generate_access_token(
        &self,
        session_id: &SessionId,
        user_id: &UserId,
    ) -> AppResult<(String, DateTime<Utc>)> {
        let expiration = Utc::now() + Duration::minutes(self.config.access_exp_minutes);

        let claims = SessionClaims {
            session_id: *session_id,
            user_id: *user_id,
            exp: expiration.timestamp(),
            typ: "access".to_string(),
        };

        let token = self
            .jwt_service
            .encode(&claims, self.config.jwt_secret.as_ref())?;

        Ok((token, expiration))
    }

    fn generate_refresh_token(
        &self,
        session_id: &SessionId,
        user_id: &UserId,
    ) -> AppResult<(String, DateTime<Utc>)> {
        let expiration = Utc::now() + Duration::days(self.config.refresh_exp_days);

        let claims = SessionClaims {
            session_id: *session_id,
            user_id: *user_id,
            exp: expiration.timestamp(),
            typ: "refresh".to_string(),
        };

        let token = self
            .jwt_service
            .encode(&claims, self.config.jwt_secret.as_ref())?;

        Ok((token, expiration))
    }

    fn hash_token(token: &str) -> String {
        let result = Sha256::digest(token.as_bytes());
        result.iter().map(|b| format!("{b:02x}")).collect()
    }
}
