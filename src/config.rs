fn normalize_base_url(url: &str) -> String {
    let mut url = url.trim().to_string();
    if !url.ends_with('/') {
        url.push('/');
    }
    url
}

/// Public base URL of this services site (e.g. `http://127.0.0.1:8080/`).
#[must_use]
pub fn public_base_url() -> String {
    std::env::var("SERVICES_PUBLIC_BASE_URL")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| normalize_base_url(&s))
        .unwrap_or_else(|| "http://127.0.0.1:8080/".to_string())
}

/// Public base URL of the identity BFF (e.g. `http://127.0.0.1:3000/`).
#[must_use]
pub fn identity_public_base_url() -> String {
    std::env::var("SERVICES_IDENTITY_PUBLIC_URL")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| normalize_base_url(&s))
        .unwrap_or_else(|| "http://127.0.0.1:3000/".to_string())
}

/// Browser origin of the identity BFF for CSP `connect-src` (no trailing slash).
#[must_use]
pub fn identity_public_origin() -> String {
    identity_public_base_url().trim_end_matches('/').to_string()
}

/// Public base URL of the cart service for navbar links.
#[must_use]
pub fn cart_public_base_url() -> String {
    std::env::var("SERVICES_CART_PUBLIC_URL")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| normalize_base_url(&s))
        .unwrap_or_else(|| "http://127.0.0.1:8084/".to_string())
}

/// Public base URL of the contact service for inquiry links.
#[must_use]
pub fn contact_public_base_url() -> String {
    std::env::var("SERVICES_CONTACT_PUBLIC_URL")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| normalize_base_url(&s))
        .unwrap_or_else(|| "http://127.0.0.1:8083/".to_string())
}

#[must_use]
pub fn contact_us_url() -> String {
    let base = contact_public_base_url();
    format!("{}contact", base.trim_end_matches('/'))
}
