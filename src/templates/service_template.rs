//! [`ServiceTemplate`].

use askama::Template;
use sigma_theme::nav::{NavEntry, SiteHeader};

#[derive(Template)]
#[template(path = "service.html")]
pub(crate) struct ServiceTemplate<'a> {
    pub(crate) slug: &'a str,
    pub(crate) title: &'a str,
    pub(crate) body: &'a str,
    pub(crate) services: &'static [NavEntry],
    pub(crate) contact_us_url: String,
    pub(crate) site_header: SiteHeader,
    pub(crate) site_nav: String,
    pub(crate) copyright_years: String,
}
