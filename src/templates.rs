use askama::Template;

use crate::config;
use crate::content::{ServiceEntry, sorted_entries};
use sigma_theme::copyright_years;
use sigma_theme::nav::{Breadcrumb, SiteHeader};
use sigma_theme::site_nav::{AppSiteNav, contact_us_url, render_app_site_nav};

fn page_header(brand: &str) -> SiteHeader {
    SiteHeader::new(brand)
}

fn site_nav(return_path: &str) -> Result<String, askama::Error> {
    render_app_site_nav(&AppSiteNav {
        identity_base: &config::identity_public_base_url(),
        app_base: &config::public_base_url(),
        contact_base: &config::contact_public_base_url(),
        cart_url: &config::cart_public_base_url(),
        cart_count: 0,
        return_path,
        show_cart: true,
        show_contact_us: false,
        leading_html: "",
    })
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    services: Vec<ServiceCard>,
    contact_us_url: String,
    site_header: SiteHeader,
    site_nav: String,
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
    site_header: SiteHeader,
    site_nav: String,
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
        contact_us_url: contact_us_url(
            &config::contact_public_base_url(),
            &config::public_base_url(),
            "/",
        ),
        site_header: page_header("Sigma Services"),
        site_nav: site_nav("/")?,
        copyright_years: copyright_years(),
    }
    .render()
}

/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_service_html(service: &ServiceEntry) -> Result<String, askama::Error> {
    let return_path = format!("/service/{}", service.slug);
    ServiceTemplate {
        slug: service.slug.clone(),
        title: service.title.clone(),
        body: service.body_html.clone(),
        services: nav_entries(),
        contact_us_url: contact_us_url(
            &config::contact_public_base_url(),
            &config::public_base_url(),
            &return_path,
        ),
        site_header: page_header("Sigma Services")
            .with_breadcrumb(Breadcrumb::link("/", "Services"))
            .with_breadcrumb(Breadcrumb::current(service.title.clone())),
        site_nav: site_nav(&return_path)?,
        copyright_years: copyright_years(),
    }
    .render()
}
