use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use time::Duration;

struct CookieConfig {
    name: String,
    max_age: i64,
}

pub struct CookieService {
    identity: CookieConfig,
    access: CookieConfig,
    refresh: CookieConfig,
}

impl CookieService {
    pub fn new() -> Self {
        Self {
            identity: CookieConfig {
                name: "identity-token".to_string(),
                max_age: 60,
            },
            access: CookieConfig {
                name: "access-token".to_string(),
                max_age: 300,
            },
            refresh: CookieConfig {
                name: "refresh-token".to_string(),
                max_age: 604800,
            },
        }
    }

    fn create_cookie<'a>(name: String, value: String, max_age_seconds: i64) -> Cookie<'a> {
        Cookie::build((name, value))
            .path("/")
            .max_age(Duration::seconds(max_age_seconds))
            .http_only(true)
            .secure(false)
            .same_site(SameSite::Lax)
            .build()
    }

    fn get_cookie(cookies: &CookieJar, name: &str) -> Option<String> {
        cookies.get(name).map(|cookie| cookie.value().to_string())
    }

    fn remove_cookie<'a>(name: String) -> Cookie<'a> {
        Cookie::build((name, ""))
            .path("/")
            .max_age(Duration::ZERO)
            .http_only(true)
            .secure(false)
            .same_site(SameSite::Lax)
            .build()
    }

    pub fn invoke_identity<'a>(&self, token: String) -> Cookie<'a> {
        Self::create_cookie(self.identity.name.clone(), token, self.identity.max_age)
    }

    pub fn invoke_access_token<'a>(&self, token: String) -> Cookie<'a> {
        Self::create_cookie(self.access.name.clone(), token, self.access.max_age)
    }

    pub fn invoke_refresh_token<'a>(&self, token: String) -> Cookie<'a> {
        Self::create_cookie(self.refresh.name.clone(), token, self.refresh.max_age)
    }

    pub fn get_identity_token(&self, cookies: &CookieJar) -> Option<String> {
        Self::get_cookie(cookies, &self.identity.name.clone())
    }

    pub fn get_access_token(&self, cookies: &CookieJar) -> Option<String> {
        Self::get_cookie(cookies, &self.access.name.clone())
    }

    pub fn get_refresh_token(&self, cookies: &CookieJar) -> Option<String> {
        Self::get_cookie(cookies, &self.refresh.name.clone())
    }

    pub fn revoke_identity_token<'a>(&self) -> Cookie<'a> {
        Self::remove_cookie(self.identity.name.clone())
    }

    pub fn revoke_access_token<'a>(&self) -> Cookie<'a> {
        Self::remove_cookie(self.access.name.clone())
    }

    pub fn revoke_refresh_token<'a>(&self) -> Cookie<'a> {
        Self::remove_cookie(self.refresh.name.clone())
    }

    pub fn revoke_all_auth_cookies(&self) -> CookieJar {
        CookieJar::new()
            .add(self.revoke_identity_token())
            .add(self.revoke_access_token())
            .add(self.revoke_refresh_token())
    }
}
