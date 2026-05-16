use rok_auth::Claims;

pub struct UserPolicy;

impl UserPolicy {
    pub fn can_view_all(claims: &Claims) -> bool {
        claims.has_role("admin")
    }

    pub fn can_view(claims: &Claims, target_user_id: i64) -> bool {
        claims.has_role("admin") || claims.sub == target_user_id.to_string()
    }

    pub fn can_update(claims: &Claims, target_user_id: i64) -> bool {
        claims.has_role("admin") || claims.sub == target_user_id.to_string()
    }

    pub fn can_delete(claims: &Claims) -> bool {
        claims.has_role("admin")
    }
}
