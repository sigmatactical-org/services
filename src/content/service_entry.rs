//! [`ServiceEntry`].

/// One professional service offering: slug, display metadata, and rendered body.
#[derive(Debug, Clone)]
pub struct ServiceEntry {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub order: i32,
    pub body_html: String,
}
