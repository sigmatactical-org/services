mod index_template;
mod service_card;
mod service_template;
pub(crate) use index_template::IndexTemplate;
pub(crate) use service_card::ServiceCard;
pub(crate) use service_template::ServiceTemplate;

use std::sync::OnceLock;

use askama::Template;

use crate::config;
use crate::content::{ServiceEntry, sorted_entries};
use sigma_theme::copyright_years;
use sigma_theme::nav::{Breadcrumb, NavEntry};
use sigma_theme::site_nav::{SiteChrome, contact_us_url};

fn chrome() -> SiteChrome {
    SiteChrome {
        title: "Services".to_string(),
        identity_base: config::identity_public_base_url(),
        app_base: config::public_base_url(),
        contact_base: config::contact_public_base_url(),
        cart_url: config::cart_public_base_url(),
        show_cart: true,
    }
}

/// Sidebar links to every service, built once from the static registry.
fn nav_entries() -> &'static [NavEntry] {
    static NAV: OnceLock<Vec<NavEntry>> = OnceLock::new();
    NAV.get_or_init(|| {
        sorted_entries()
            .into_iter()
            .map(|s| NavEntry {
                slug: s.slug.clone(),
                title: s.title.clone(),
            })
            .collect()
    })
}

/// Landing-page cards for every service, built once from the static registry.
fn service_cards() -> &'static [ServiceCard] {
    static CARDS: OnceLock<Vec<ServiceCard>> = OnceLock::new();
    CARDS.get_or_init(|| {
        sorted_entries()
            .into_iter()
            .map(|s| ServiceCard {
                slug: &s.slug,
                title: &s.title,
                summary: &s.summary,
            })
            .collect()
    })
}

/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_index_html() -> Result<String, askama::Error> {
    let chrome = chrome();
    IndexTemplate {
        services: service_cards(),
        contact_us_url: contact_us_url(
            &config::contact_public_base_url(),
            &config::public_base_url(),
            "/",
        ),
        site_header: chrome.page_header(None),
        site_nav: chrome.site_nav("/", 0)?,
        copyright_years: copyright_years(),
    }
    .render()
}

/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_service_html(service: &ServiceEntry) -> Result<String, askama::Error> {
    let chrome = chrome();
    let return_path = format!("/service/{}", service.slug);
    ServiceTemplate {
        slug: &service.slug,
        title: &service.title,
        body: &service.body_html,
        services: nav_entries(),
        contact_us_url: contact_us_url(
            &config::contact_public_base_url(),
            &config::public_base_url(),
            &return_path,
        ),
        site_header: chrome
            .page_header(None)
            .with_breadcrumb(Breadcrumb::link("/", "Services"))
            .with_breadcrumb(Breadcrumb::current(service.title.as_str())),
        site_nav: chrome.site_nav(&return_path, 0)?,
        copyright_years: copyright_years(),
    }
    .render()
}
