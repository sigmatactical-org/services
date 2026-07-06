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

fn normalize_base_url(url: &str) -> String {
    let mut url = url.trim().to_string();
    if !url.ends_with('/') {
        url.push('/');
    }
    url
}
