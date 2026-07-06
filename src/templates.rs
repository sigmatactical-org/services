use askama::Template;

use crate::config;
use crate::content::{ServiceEntry, sorted_entries};
use sigma_theme::copyright_years;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    services: Vec<ServiceCard>,
    contact_us_url: String,
    copyright_years: String,
}

#[derive(Template)]
#[template(path = "service.html")]
struct ServiceTemplate {
    slug: String,
    title: String,
    body: String,
    services: Vec<NavEntry>,
    contact_us_url: String,
    copyright_years: String,
}

#[derive(Clone)]
struct ServiceCard {
    slug: String,
    title: String,
    summary: String,
}

#[derive(Clone)]
struct NavEntry {
    slug: String,
    title: String,
}

fn nav_entries() -> Vec<NavEntry> {
    sorted_entries()
        .into_iter()
        .map(|s| NavEntry {
            slug: s.slug.clone(),
            title: s.title.clone(),
        })
        .collect()
}

fn service_cards() -> Vec<ServiceCard> {
    sorted_entries()
        .into_iter()
        .map(|s| ServiceCard {
            slug: s.slug.clone(),
            title: s.title.clone(),
            summary: s.summary.clone(),
        })
        .collect()
}

/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_index_html() -> Result<String, askama::Error> {
    IndexTemplate {
        services: service_cards(),
        contact_us_url: config::contact_us_url(),
        copyright_years: copyright_years(),
    }
    .render()
}

/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_service_html(service: &ServiceEntry) -> Result<String, askama::Error> {
    ServiceTemplate {
        slug: service.slug.clone(),
        title: service.title.clone(),
        body: service.body_html.clone(),
        services: nav_entries(),
        contact_us_url: config::contact_us_url(),
        copyright_years: copyright_years(),
    }
    .render()
}
