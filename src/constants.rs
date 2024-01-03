pub const IGNORE_AUTH_ROUTES: [&str; 3] = ["/api/tags", "/api/users", "/api/users/login"];
pub const AUTHORIZATION: &str = "Authorization";
pub const BIND: &str = "127.0.0.1:8080";
pub mod env_key {
    pub const DATABASE_URL: &str = "DATABASE_URL";
    pub const FRONTEND_ORIGIN: &str = "FRONTEND_ORIGIN";
}

pub mod error_msg {
    pub const UNAUTHORIZED: &str = "Unauthorized";
}