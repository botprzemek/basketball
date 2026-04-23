use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::{Actor, AuthenticatedActor, IdentitySelectionActor};

pub enum AuthenticationState {
    Pending { identity: String },
    Authenticated { access: String, refresh: String },
}

#[derive(Serialize, Deserialize)]
pub struct IdentitySelectionClaims {
    pub iss: String,
    pub sub: Uuid,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
    pub jti: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct AccessClaims {
    pub iss: String,
    pub sub: Uuid,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
    pub jti: Uuid,
    pub auth_version: i32,
    pub iid: Uuid,
    pub oid: Uuid,

    pub roles: Vec<Uuid>,
    pub permissions: Vec<String>,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshClaims {
    pub iss: String,
    pub sub: Uuid,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
    pub jti: Uuid,
    pub auth_version: i32,
    pub iid: Uuid,
    pub oid: Uuid,
}

pub struct TokenService {
    issuer: String,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Claims {
    Access(AccessClaims),
    Refresh(RefreshClaims),
    Identity(IdentitySelectionClaims),
}

impl TokenService {
    pub fn new(issuer: String, secret: String) -> Self {
        let secret = secret.as_bytes();
        Self {
            issuer,
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    pub fn decode(&self, token: &str) -> anyhow::Result<Claims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&["auth:identity", "auth:access", "auth:refresh"]);
        validation.set_issuer(&[&self.issuer]);

        let decoded = decode::<Claims>(token, &self.decoding_key, &validation)?;

        Ok(decoded.claims)
    }

    pub fn issue_identity_selection(
        &self,
        account_id: Uuid,
    ) -> anyhow::Result<AuthenticationState> {
        let identity = IdentitySelectionClaims {
            iss: self.issuer.clone(),
            sub: account_id,
            aud: "auth:identity".to_string(),
            exp: chrono::Utc::now().timestamp() + 60,
            iat: chrono::Utc::now().timestamp(),
            jti: Uuid::now_v7(),
        };

        let identity = encode(&Header::default(), &identity, &self.encoding_key)?;

        Ok(AuthenticationState::Pending { identity })
    }

    pub fn issue_authentication(
        &self,
        account_id: Uuid,
        identity_id: Uuid,
        organization_id: Uuid,
    ) -> anyhow::Result<AuthenticationState> {
        let access = AccessClaims {
            iss: self.issuer.clone(),
            sub: account_id,
            aud: "auth:access".to_string(),
            exp: chrono::Utc::now().timestamp() + 900,
            iat: chrono::Utc::now().timestamp(),
            jti: Uuid::now_v7(),
            auth_version: 1,
            iid: identity_id,
            oid: organization_id,
        };
        let refresh = RefreshClaims {
            iss: self.issuer.clone(),
            sub: account_id,
            aud: "auth:refresh".to_string(),
            exp: chrono::Utc::now().timestamp() + 604800,
            iat: chrono::Utc::now().timestamp(),
            jti: Uuid::now_v7(),
            auth_version: 1,
            iid: identity_id,
            oid: organization_id,
        };

        let access = encode(&Header::default(), &access, &self.encoding_key)?;
        let refresh = encode(&Header::default(), &refresh, &self.encoding_key)?;

        Ok(AuthenticationState::Authenticated { access, refresh })
    }

    pub fn authenticate(&self, token: &str) -> anyhow::Result<Actor> {
        match self.decode(token)? {
            Claims::Identity(claims) => Ok(Actor::Selection(IdentitySelectionActor {
                account_id: claims.sub,
            })),
            Claims::Access(claims) => Ok(Actor::Authorized(AuthenticatedActor {
                account_id: claims.sub,
                identity_id: claims.iid,
                organization_id: claims.oid,
            })),
            _ => Err(anyhow::anyhow!("Invalid authentication token")),
        }
    }

    pub fn refresh(&self, token: &str) -> anyhow::Result<AuthenticationState> {
        match self.decode(token)? {
            Claims::Refresh(claims) => {
                self.issue_authentication(claims.sub, claims.iid, claims.oid)
            }
            _ => Err(anyhow::anyhow!("Invalid refresh token")),
        }
    }
}
